pub mod index;

use anyhow::Result;

use super::RenderContext;


pub(in super) trait Page {
    fn render(&self, rcontext: &RenderContext) -> Result<String>;
}