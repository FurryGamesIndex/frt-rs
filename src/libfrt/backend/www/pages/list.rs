use std::collections::HashMap;

use anyhow::Result;

use super::Page;
use super::PageRenderOutput;
use super::RenderContext;

pub struct PageList {
}

impl PageList {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for PageList {
    fn render(&self, rcontext: &RenderContext) -> Result<PageRenderOutput> {
        info!("Rendering: list");

        let mut tera_context = rcontext.make_common_tera_context()?;
        tera_context.insert("rr", "..");
        tera_context.insert("games", &rcontext.data.games);

        let s = rcontext.tera.render("list.html", &tera_context)?;

        Ok(PageRenderOutput::single_page(
            format!("{}/list.html", rcontext.lang.as_str()),
            s,
        ))
    }
}
