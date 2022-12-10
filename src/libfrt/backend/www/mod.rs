mod profile;
mod pages;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use profile::ProfileWWW;
use pages::{Page, index::PageIndex, PageRenderOutput};
use super::{Backend, BackendArguments};
use crate::error::{Error, ErrorKind};
use crate::ContextData;
use crate::profile::Profile;
use crate::i18n::LangId;

struct RenderContext<'a> {
    data: &'a ContextData,
    args: BackendArguments,
    tera: &'a tera::Tera,
    lang: LangId,
}

impl RenderContext<'_> {
    pub fn make_common_tera_context(&self) -> Result<tera::Context> {
        let ui = self.data.ui.get(&self.lang).unwrap();
        let mut tera_context = tera::Context::new();

        tera_context.insert("ui", ui);

        Ok(tera_context)
    }
}

pub struct BackendWWW {
    pub profile: ProfileWWW,
    pub tera: tera::Tera,

    pages: HashMap<String, Box<dyn Page>>,
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

impl BackendWWW {
    pub fn new(value: Option<toml::Value>) -> Result<Self> {
        let mut backend = Self {
            profile: match value {
                Some(value) => ProfileWWW::from_value(value)?,
                None => ProfileWWW::default(),
            },
            tera: tera::Tera::default(),

            pages: HashMap::new(),
        };

        for path in backend.profile.path_templates.iter() {
            info!("Loading templates from: {path}");
            backend.tera.extend(&tera::Tera::new(path)?)?;
        }

        let mut ifac = TeraIconFactory::default();
        for path in backend.profile.path_icon.iter() {
            info!("Loading icons metadatas from: {path}");
            let json = std::fs::read_to_string(
                Path::new(path).join("FGI-icons.json"))?;

            ifac.icons = serde_json::from_str(&json)?;
        }

        backend.tera.register_function("icon", ifac);

        backend.pages.insert("index".to_string(), Box::new(PageIndex::new()));

        Ok(backend)
    }
}

impl Backend for BackendWWW {
    fn render(
        &self, profile: &Profile,
        data: &ContextData,
        args: BackendArguments
    ) -> Result<BackendArguments> {
        let mut ret = BackendArguments::new();

        let output_dir = args.get("output")
            .ok_or_else(|| Error::new(ErrorKind::InvalidArgument, 
                "Missing argument 'output'"))?
            .to_owned();

        let output_dir = Path::new(&output_dir);

        let output_dir = Path::new(&output_dir);

        if std::fs::metadata(&output_dir).is_ok() {
            std::fs::remove_dir_all(&output_dir)?;
        }

        let mut render_context = RenderContext {
            data: data,
            args: args,
            tera: &self.tera,
            lang: LangId::default(),
        };

        let target = render_context.args.get("target").map_or("", String::as_str);

        let mut output = PageRenderOutput::default();

        for lang in render_context.data.ui.keys() {
            render_context.lang = lang.clone();

            output.extend(match target {
                "" => {
                    let mut out = PageRenderOutput::default();
                    for page in self.pages.values() {
                        out.extend(page.render(&render_context)?);
                    }
                    out
                },
                _ => match self.pages.get(target) {
                    Some(page) => page.render(&render_context)?,
                    None => return Err(Error::new(ErrorKind::InvalidArgument,
                        format!("Unsupported argument target '{}'", target).as_str()).into())
                }
            });
        }

        for (fnm, file) in output.pages.iter() {
            match file {
                pages::File::Regular(contents) => {
                    std::fs::write(output_dir.join(fnm), contents)?;
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