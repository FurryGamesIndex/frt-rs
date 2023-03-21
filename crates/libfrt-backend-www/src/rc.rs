use anyhow::Result;

use crate::BackendWWW;
use libfrt::{ContextData, i18n::LangId};

pub struct RenderContext<'a> {
    pub backend: &'a BackendWWW,

    pub data: &'a ContextData,
    pub lang: LangId,
}

impl RenderContext<'_> {
    pub fn ui(&self, k: impl AsRef<str>) -> &str {
        todo!()
    }
    pub fn ui_raw(&self, k: impl AsRef<str>) -> Option<toml::Value> {
        todo!()
    }
    pub fn icon(&self, scope: impl AsRef<str>, name: impl AsRef<str>) -> &str {
        todo!()
    }
    pub fn res(&self, rr: impl AsRef<str>, scope: impl AsRef<str>, name: impl AsRef<str>) -> &str {
        todo!()
    }
}