pub mod index;

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