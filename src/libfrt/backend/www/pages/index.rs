

use anyhow::Result;
use tera::Tera;

use super::Page;
use super::RenderContext;

pub struct PageIndex {

}

impl PageIndex {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for PageIndex {
    fn render(&self, rcontext: &RenderContext) -> Result<String> {
        todo!()
    }
}