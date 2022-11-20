use anyhow::Result;

use super::{Backend, BackendArguments};
use crate::ContextData;
use crate::profile::Profile;

pub struct BackendWWW {

}

impl BackendWWW {
    pub fn new() -> Self {
        Self {}
    }
}

impl Backend for BackendWWW {
    fn render(
        &self, profile: &Profile,
        data: &ContextData,
        options: BackendArguments
    ) -> Result<BackendArguments> {
        todo!()
    }
}