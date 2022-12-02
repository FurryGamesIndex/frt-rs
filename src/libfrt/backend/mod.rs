pub mod neutral;
pub mod www;

use std::collections::HashMap;

use anyhow::Result;

use crate::ContextData;
use crate::profile::Profile;

pub type BackendArguments = HashMap<String, String>;

pub trait Backend {
    fn render(
        &self, profile: &Profile,
        data: &ContextData,
        args: BackendArguments
    ) -> Result<BackendArguments>;
}