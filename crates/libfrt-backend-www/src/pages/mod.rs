pub mod misc;
//pub mod list;

use std::collections::HashMap;

use anyhow::Result;

use super::RenderContext;

pub enum File {
    Regular(String),
    Symlink(String)
}

#[derive(Default)]
pub struct PageRenderOutput {
    pub pages: HashMap<String, File>
}

impl PageRenderOutput {
    pub fn single_page(path: String, content: String) -> Self {
        let mut ret = Self::default();
        ret.pages.insert(path, File::Regular(content));
        ret
    }

    pub fn extend(&mut self, b: PageRenderOutput) {
        self.pages.extend(b.pages);
    }
}

pub(in super) trait Page {
    fn render(&self, rcontext: &RenderContext) -> Result<PageRenderOutput>;
}

#[derive(Default)]
pub struct CVMeta {
    pub title: String,
    pub keywords: String, /* TODO */
    pub description: String,
    pub image: String, /* TODO */
}

#[derive(Default)]
pub struct TemplateCommonVariables {
    meta: Option<CVMeta>,
    noindex: bool,
    extra_styles: Option<String>,
}

macro_rules! template {
    ($file:literal, $name:ident $(, $custom_field:ty)?) => {
        #[derive(Template)]
        #[template(path = $file)]
        pub(crate) struct $name<'a> {
            rr: &'static str,
            rc: &'a RenderContext<'a>,
            g: $crate::pages::TemplateCommonVariables,
            $(c: $custom_field,)?
        }

        impl<'a> $name<'a> {
            fn new(rc: &'a RenderContext) -> $name<'a> {
                Self {
                    rr: "..",
                    rc,
                    g: $crate::pages::TemplateCommonVariables::default(),
                    $(c: $custom_field::default(),)?
                }
            }
        }
    }
}

use template;