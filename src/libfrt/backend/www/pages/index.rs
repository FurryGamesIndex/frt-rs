

use anyhow::Result;
use tera::Tera;

use super::Page;
use super::PageRenderOutput;
use super::RenderContext;

pub struct PageIndex {

}

impl PageIndex {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for PageIndex {
    fn render(&self, rcontext: &RenderContext) -> Result<PageRenderOutput> {
        let mut tera_context = rcontext.make_common_tera_context()?;
        tera_context.insert("rr", "..");

        let s = rcontext.tera.render("index.html", &tera_context)?;

        Ok(PageRenderOutput::single_page(
            format!("{}/index.html", rcontext.lang.as_str()), s))
    }
}