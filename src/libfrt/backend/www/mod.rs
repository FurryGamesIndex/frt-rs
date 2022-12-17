mod profile;
mod pages;
mod entries;
mod stylesheet;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use profile::ProfileWWW;
use stylesheet::Stylesheets;
use pages::{Page, misc::PageMisc, PageRenderOutput};
use self::entries::game::GameWWW;
use self::pages::list::PageList;
use super::{Backend, BackendArguments};
use crate::error::{Error, ErrorKind};
use crate::ContextData;
use crate::profile::Profile;
use crate::i18n::LangId;
use crate::utils::fs::{copy_dir, get_mtime, ensure_dir};

struct RenderContext<'a> {
    games: &'a HashMap<String, GameWWW>,

    data: &'a ContextData,
    args: &'a BackendArguments,
    tera: &'a tera::Tera,
    lang: LangId,
}

impl RenderContext<'_> {
    pub fn make_common_tera_context(&self) -> Result<tera::Context> {
        let ui = self.data.ui.get(&self.lang).unwrap();
        let mut tera_context = tera::Context::new();

        tera_context.insert("ui", ui);
        tera_context.insert("lang", &self.lang.as_str());
        tera_context.insert("lang_without_region", &self.lang.as_str_noregion());
        tera_context.insert("lang_unix_style", &self.lang.as_unix());
        tera_context.insert("lang_bcp47", &self.lang.as_bcp47_short());

        Ok(tera_context)
    }
}

pub struct BackendWWW {
    pub profile: ProfileWWW,
    pub tera: tera::Tera,

    stylesheets: Stylesheets,
    pages: HashMap<String, Box<dyn Page>>,

    games: HashMap<String, GameWWW>,
    langs: Vec<LangId>,
}

#[derive(Default)]
struct TeraIconFactory {
    pub icons: HashMap<String, u32>,
}

impl tera::Function for TeraIconFactory {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let scope = match args.get("scope") {
            Some(var) => var.as_str().unwrap_or("misc"),
            None => "misc",
        };

        let name = match args.get("name") {
            Some(var) => match var.as_str() {
                Some(s) => s,
                None => return Err("Argument 'name' should be str".into()),
            },
            None => return Err("Argument 'name' should not be empty".into()),
        };

        let id = match self.icons.get(format!("{}-{}", scope, name).as_str()) {
            Some(id) => id,
            None => match self.icons.get(format!("{}-fallback", scope).as_str()) {
                Some(id) => id,
                None => match self.icons.get("misc-fallback") {
                    Some(id) => id,
                    None => return Err("No global fallback, check icon".into()),
                },
            },
        };

        Ok(format!("<span class=\"icon\" data-icon=\"&#{};\" aria-hidden=\"true\"></span>", id).into())
    }
}

struct TeraRes {
    pub profile: ProfileWWW,
    pub stylesheets: Stylesheets,
}

impl TeraRes {
    fn find_local_file(&self, dir: &Vec<String>, rr: &str, file: &str, output_prefix: &str) -> Result<String> {
        for i in dir.iter().rev() {
            let p = Path::new(i).join(file);
            if p.is_file() {
                let mtime = get_mtime(p)?;

                return Ok(format!("{}/{}{}?hc=uquery&t={}", rr, output_prefix, file, mtime).into());
            }
        }

        Err(Error::new(ErrorKind::NotExist, "File not found").into())
    }

    fn find_stylesheet_file(&self, rr: &str, file: &str) -> Result<String> {
        match self.stylesheets.sheets.get(file) {
            Some(ss) => {
                Ok(format!("{}/{}?hc=uquery&t={}", rr, file, ss.mtime).into())
            },
            None => Err(Error::new(ErrorKind::NotExist, "File not found").into()),
        }
    }
}

impl tera::Function for TeraRes {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let rr = match args.get("rr") {
            Some(var) => var.as_str().unwrap_or(""),
            None => "",
        };

        let scope = match args.get("scope") {
            Some(var) => var.as_str().unwrap_or(""),
            None => "",
        };

        let file = match args.get("path") {
            Some(var) => match var.as_str() {
                Some(s) => s,
                None => return Err("Argument 'path' should be str".into()),
            }
            None => return Err("Argument 'path' should not be empty".into()),
        };

        let rf = match scope {
            "" | "root" => {
                self.find_local_file(&self.profile.path_static_layers, rr, file, "")
            },
            "icons" => {
                self.find_local_file(&self.profile.path_icon, rr, file, "icons/")
            },
            "styles" => {
                self.find_stylesheet_file(rr, file)
            },
            _ => return Err("Argument 'scope' should be str".into()),
        };

        match rf {
            Ok(f) => Ok(f.into()),
            Err(e) => Err(format!("File not found: {}:{}. Err: {}", scope, file, e).into()),
        }
    }
}

impl BackendWWW {
    pub fn new(value: Option<toml::Value>) -> Result<Self> {
        let mut backend = Self {
            profile: match value {
                Some(value) => ProfileWWW::from_value(value)?,
                None => ProfileWWW::default(),
            },
            tera: tera::Tera::default(),

            stylesheets: Stylesheets::default(),
            pages: HashMap::new(),

            games: HashMap::new(),
            langs: Vec::new(),
        };

        for path in backend.profile.path_templates.iter() {
            info!("Loading templates from: {path}");
            backend.tera.extend(&tera::Tera::new(path)?)?;
        }

        for path in backend.profile.path_stylesheets.iter() {
            info!("Loading stylesheets from: {path}");
            backend.stylesheets.extend(Stylesheets::from_path(Path::new(path))?);
        }

        let mut ifac = TeraIconFactory::default();
        for path in backend.profile.path_icon.iter() {
            info!("Loading icons metadatas from: {path}");
            let json = std::fs::read_to_string(
                Path::new(path).join("FGI-icons.json"))?;

            ifac.icons = serde_json::from_str(&json)?;
        }
        backend.tera.register_function("icon", ifac);

        let res = TeraRes {
            profile: backend.profile.clone(),
            stylesheets: backend.stylesheets.clone(),
        };
        backend.tera.register_function("res", res);

        backend.pages.insert("misc".to_string(), Box::new(PageMisc::new()));
        backend.pages.insert("list".to_string(), Box::new(PageList::new()));

        Ok(backend)
    }
}

impl Backend for BackendWWW {
    fn resync(
        &mut self,
        profile: &Profile,
        data: &mut ContextData,
        args: &BackendArguments
    ) -> Result<()> {
        info!("Re-syncing backend data");

        self.langs = data.ui.keys().map(|i| i.to_owned()).collect();
        if !data.ui.contains_key(&LangId::default()) {
            self.langs.push(LangId::default());
        }

        for game in data.games.values_mut() {
            if game.dirty {
                self.games.insert(game.id.clone(), GameWWW::from_game(game, &self.langs)?);

                game.dirty = false;
            }
        }

        Ok(())
    }

    fn render(
        &self,
        profile: &Profile,
        data: &ContextData,
        args: &BackendArguments
    ) -> Result<BackendArguments> {
        let mut ret = BackendArguments::default();

        let output_dir = args.get_string("output")
            .ok_or_else(|| Error::new(ErrorKind::InvalidArgument, 
                "Missing argument 'output'"))?;

        let output_dir = Path::new(&output_dir);

        if std::fs::metadata(&output_dir).is_ok() {
            std::fs::remove_dir_all(&output_dir)?;
        }

        let mut render_context = RenderContext {
            games: &self.games,
            data: data,
            args: args,
            tera: &self.tera,
            lang: LangId::default(),
        };

        let target = render_context.args.get_string("target").unwrap_or(String::new());

        let mut output = PageRenderOutput::default();

        for lang in self.langs.iter() {
            render_context.lang = lang.clone();
            info!("Render starting, lang: {}", render_context.lang.as_str());

            output.extend(match target.as_str() {
                "" => {
                    let mut out = PageRenderOutput::default();
                    for page in self.pages.values() {
                        out.extend(page.render(&render_context)?);
                    }
                    out
                },
                _ => match self.pages.get(target.as_str()) {
                    Some(page) => page.render(&render_context)?,
                    None => return Err(Error::new(ErrorKind::InvalidArgument,
                        format!("Unsupported argument target '{}'", target).as_str()).into())
                }
            });
        }

        for src in self.profile.path_static_layers.iter() {
            info!("Copy static layer '{}'", src);
            copy_dir(src, output_dir)?;
        }

        for src in self.profile.path_icon.iter() {
            info!("Copy icon layer '{}'", src);
            let output_dir = output_dir.join("icons");
            copy_dir(src, output_dir)?;
        }

        for (fnm, f) in self.stylesheets.sheets.iter() {
            info!("Write stylesheet '{}'", fnm);

            let p = output_dir.join(fnm);
            ensure_dir(&p)?;

            std::fs::write(p, &f.contents)?;
        }

        info!("Write generated files");
        for (fnm, file) in output.pages.iter() {
            match file {
                pages::File::Regular(contents) => {
                    let f = output_dir.join(fnm);
                    ensure_dir(&f)?;

                    std::fs::write(f, contents)?;
                },
                pages::File::Symlink(original) => {
                    #[cfg(unix)]
                    std::os::unix::fs::symlink(original, output_dir.join(fnm))?;
                    #[cfg(windows)]
                    error!("Symlink not currently supported on Windows platform. Ignored.");
                },
            }
            
        }

        Ok(ret)
    }
}