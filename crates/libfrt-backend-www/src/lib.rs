mod profile;
mod pages;
mod entries;
mod stylesheet;
mod rc;

#[macro_use] extern crate log;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use profile::ProfileWWW;
use stylesheet::Stylesheets;
use pages::{Page, misc::PageMisc, PageRenderOutput};
use entries::game::GameWWW;
use libfrt::backend::{Backend, BackendArguments};
use libfrt::error::{Error, ErrorKind};
use libfrt::ContextData;
use libfrt::profile::Profile;
use libfrt::i18n::LangId;
use libfrt::utils::fs::{copy_dir, ensure_dir, make_dir};
use crate::rc::RenderContext;

enum OutputMode {
    NoOutput,
    Filesystem(PathBuf),
}

pub struct BackendWWW {
    pub profile: ProfileWWW,

    output: OutputMode,
    target: String,

    stylesheets: Stylesheets,
    pages: HashMap<String, Box<dyn Page>>,
    langs: Vec<LangId>,

    games: HashMap<String, GameWWW>,
}

impl BackendWWW {
    pub fn new(value: Option<toml::Value>) -> Result<Self> {
        let mut backend = Self {
            profile: match value {
                Some(value) => ProfileWWW::from_value(value)?,
                None => ProfileWWW::default(),
            },

            output: OutputMode::NoOutput,
            target: String::new(),

            stylesheets: Stylesheets::default(),
            pages: HashMap::new(),

            games: HashMap::new(),
            langs: Vec::new(),
        };

        for path in backend.profile.path_stylesheets.iter() {
            info!("Loading stylesheets from: {path}");
            backend.stylesheets.extend(Stylesheets::from_path(Path::new(path))?);
        }

        for path in backend.profile.path_icon.iter() {
            info!("Loading icons metadatas from: {path}");
            let json = std::fs::read_to_string(
                Path::new(path).join("FGI-icons.json"))?;

            //ifac.icons = serde_json::from_str(&json)?;
        }

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

        if args.get_bool("fs_output") {
            let output_dir = args.get_string("output")
            .ok_or_else(|| Error::new(ErrorKind::InvalidArgument, 
                "Missing argument 'output'"))?;

            self.output = OutputMode::Filesystem(Path::new(&output_dir).into());
        }

        self.target = args.get_string("target").unwrap_or(String::new());

        self.langs = data.ui.keys().map(|i| i.to_owned()).collect();
        if !data.ui.contains_key(&LangId::default()) {
            self.langs.push(LangId::default());
        }

        //for game in data.games.values_mut() {
        //    if game.dirty {
        //        self.games.insert(game.id.clone(), GameWWW::from_game(game, &self)?);
        //        //game.dirty = false;
        //    }
        //}

        Ok(())
    }

    fn render(
        &self,
        profile: &Profile,
        data: &ContextData
    ) -> Result<BackendArguments> {
        let mut ret = BackendArguments::default();

        if let OutputMode::Filesystem(output_dir) = &self.output {
            if std::fs::metadata(&output_dir).is_ok() {
                std::fs::remove_dir_all(&output_dir)?;
            }
        }

        let mut render_context = RenderContext {
            backend: &self,
            data: data,
            lang: LangId::default(),
        };

        let mut output = PageRenderOutput::default();

        for lang in self.langs.iter() {
            render_context.lang = lang.clone();
            info!("Render starting, lang: {}", render_context.lang.as_str());

            output.extend(match self.target.as_str() {
                "" => {
                    let mut out = PageRenderOutput::default();
                    for page in self.pages.values() {
                        out.extend(page.render(&render_context)?);
                    }
                    out
                },
                _ => match self.pages.get(self.target.as_str()) {
                    Some(page) => page.render(&render_context)?,
                    None => return Err(Error::new(ErrorKind::InvalidArgument,
                        format!("Unsupported argument target '{}'", self.target).as_str()).into())
                }
            });
        }

        if let OutputMode::Filesystem(output_dir) = &self.output {
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
        } else {
            todo!("Copy rendered pages into ret for NoOutput mode");
        }

        Ok(ret)
    }
}