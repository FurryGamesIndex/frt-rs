use anyhow::Result;

use super::template;
use super::Page;
use super::PageRenderOutput;
use super::RenderContext;
use crate::entries::game::GameWWW;
use askama::Template;

#[derive(Default)]
struct ListTemplatePriv<'a> {
    games: Vec<&'a GameWWW>,
}

template!("list.html", ListTemplate, "..", ListTemplatePriv<'a>);

pub struct PageList {}

impl PageList {
    pub fn new() -> Self {
        Self {}
    }

    fn render_list<F>(
        &self,
        template: &mut ListTemplate,
        compare: F,
        fn_suffix: &str,
    ) -> Result<PageRenderOutput>
    where
        F: FnMut(&&GameWWW, &&GameWWW) -> std::cmp::Ordering,
    {
        template.c.games = template.rc.backend.games.values().collect();
        template.c.games.sort_unstable_by(compare);

        Ok(PageRenderOutput::single_page(
            format!("{}/list{}.html", template.rc.lang.as_str(), fn_suffix),
            template.render()?,
        ))
    }
}

impl Page for PageList {
    fn render(&self, rc: &RenderContext) -> Result<PageRenderOutput> {
        info!("Rendering: list");

        let mut template = ListTemplate::new(rc);

        let mut ret = PageRenderOutput::default();

        ret.extend(self.render_list(&mut template, |a, b| a.orig.id.cmp(&b.orig.id), "")?);

        Ok(ret)
    }
}
