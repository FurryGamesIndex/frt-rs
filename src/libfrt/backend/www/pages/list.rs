use anyhow::Result;

use crate::backend::www::entries::game::GameWWW;
use crate::i18n::LangId;
use super::Page;
use super::PageRenderOutput;
use super::RenderContext;

pub struct PageList {
}

impl PageList {
    pub fn new() -> Self {
        Self {}
    }

    fn render_list<F>(&self, rcontext: &RenderContext, compare: F) -> Result<PageRenderOutput>
    where
        F: FnMut(&&GameWWW, &&GameWWW) -> std::cmp::Ordering
    {
        let mut tera_context = rcontext.make_common_tera_context()?;
        tera_context.insert("rr", "..");

        let mut games: Vec<_> = rcontext.games.values().collect();
        games.sort_unstable_by(compare);

        let games: Vec<_> = games.into_iter().map(|g| {
            g.1.get(&rcontext.lang).unwrap_or(g.1.get(&LangId::default()).unwrap())
        }).collect();

        tera_context.insert("games", &games);

        let s = rcontext.tera.render("list.html", &tera_context)?;

        Ok(PageRenderOutput::single_page(
            format!("{}/list.html", rcontext.lang.as_str()),
            s,
        ))
    }
}

impl Page for PageList {
    fn render(&self, rcontext: &RenderContext) -> Result<PageRenderOutput> {
        info!("Rendering: list");

        let mut ret = PageRenderOutput::default();

        ret.extend(self.render_list(rcontext, |a, b| {
            a.0.cmp(&b.0)
        })?);

        Ok(ret)
    }
}
