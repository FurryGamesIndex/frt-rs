use std::collections::HashMap;

use anyhow::Result;

use super::Page;
use super::PageRenderOutput;
use super::RenderContext;
use super::TemplateCommonVariables;
use askama::Template;
use libfrt::i18n::LangId;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    rr: &'static str,
    rc: &'a RenderContext<'a>,
    g: TemplateCommonVariables,
}

impl<'a> IndexTemplate<'a> {
    fn new(rc: &'a RenderContext) -> IndexTemplate<'a> {
        Self {
            rr: "..",
            rc,
            g: TemplateCommonVariables::default(),
        }
    }
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct C404Template<'a> {
    rr: &'static str,
    rc: &'a RenderContext<'a>,
    g: TemplateCommonVariables,
}

impl<'a> C404Template<'a> {
    fn new(rc: &'a RenderContext) -> C404Template<'a> {
        Self {
            rr: "/",
            rc,
            g: TemplateCommonVariables::default(),
        }
    }
}

pub struct PageMisc {}

impl PageMisc {
    pub fn new() -> Self {
        Self {}
    }

    fn render_page(
        rcontext: &RenderContext,
        template: impl Template,
        output_fn: &str,
        i18n_support: bool,
    ) -> Result<PageRenderOutput> {
        if !i18n_support && rcontext.lang != LangId::default() {
            return Ok(PageRenderOutput::default());
        }

        let s = template.render()?;

        if i18n_support {
            Ok(PageRenderOutput::single_page(
                format!("{}{}", rcontext.lang.as_str(), output_fn),
                s,
            ))
        } else {
            Ok(PageRenderOutput::single_page(format!("{}", output_fn), s))
        }
    }
}

impl Page for PageMisc {
    fn render(&self, rc: &RenderContext) -> Result<PageRenderOutput> {
        let mut ret = PageRenderOutput::default();

        ret.extend(Self::render_page(
            rc,
            IndexTemplate::new(rc),
            "/index.html",
            true,
        )?);
        //ret.extend(Self::render_page(rc, LanguagesTemplate::new(rc), "languages.html", false)?);
        ret.extend(Self::render_page(rc, C404Template::new(rc), "404.html", false)?);

        Ok(ret)
    }
}
