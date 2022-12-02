mod profile;
mod pages;

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;

use profile::ProfileWWW;
use pages::{Page, index::PageIndex};
use super::{Backend, BackendArguments};
use crate::error::{Error, ErrorKind};
use crate::ContextData;
use crate::profile::Profile;

struct RenderContext<'a> {
    data: &'a ContextData,
    args: BackendArguments
}

pub struct BackendWWW {
    pub profile: ProfileWWW,
    pub tera: tera::Tera,

    pages: HashMap<String, Rc<dyn Page>>,

    page_index: Rc<PageIndex>,
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

            page_index: Rc::new(PageIndex::new()),
        };

        for path in backend.profile.path_templates.iter() {
            info!("Loading templates from: {path}");
            backend.tera.extend(&tera::Tera::new(path)?)?;
        }

        backend.pages.insert("index".to_string(), backend.page_index.clone());

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

        let output_dir = args.get("output").ok_or_else(
            || Error::new(ErrorKind::InvalidArgument, "missing argument 'output'"))?;

        if std::fs::metadata(&output_dir).is_ok() {
            std::fs::remove_dir_all(&output_dir)?;
        }

        let render_context = RenderContext {
            data: data,
            args: args
        };

        let target = render_context.args.get("target").map_or("", String::as_str);
        
        let result_str = match target {
            _ | "" | "output" => {
                Ok("".to_owned())
            },
            "index" => self.page_index.render(&render_context),
        }?;

        ret.insert("result".to_owned(), result_str);

        Ok(ret)
    }
}