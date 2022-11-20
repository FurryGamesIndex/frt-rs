pub mod neutral;
pub mod www;

use std::collections::HashMap;

use anyhow::Result;

use crate::ContextData;
use crate::profile::Profile;

type BackendArguments = HashMap<String, String>;

pub trait Backend {
    fn render(
        &self, profile: &Profile,
        data: &ContextData,
        options: BackendArguments
    ) -> Result<BackendArguments>;
}