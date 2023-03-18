use std::collections::HashMap;

use anyhow::Result;

use crate::ContextData;
use crate::profile::Profile;

#[derive(Default)]
pub struct BackendArguments(HashMap<String, serde_json::Value>);

impl BackendArguments {
    pub fn set_string(&mut self, k: String, v: String) {
        self.0.insert(k, serde_json::Value::String(v));
    }

    pub fn set_bool(&mut self, k: String, v: bool) {
        self.0.insert(k, serde_json::Value::Bool(v));
    }

    pub fn get_string<K>(&self, k: K) -> Option<String>
    where
        K: AsRef<str>
    {
        self.0.get(k.as_ref()).map(|v| {
            match v {
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::String(s) => s.clone(),
                _ => String::new()
            }
        })
    }

    pub fn get_bool<K>(&self, k: K) -> bool
    where
        K: AsRef<str>
    {
        self.0.get(k.as_ref()).map(|v| {
            match v {
                serde_json::Value::Bool(b) => *b,
                _ => false
            }
        }).unwrap_or(false)
    }
}

pub trait Backend {
    fn resync(
        &mut self,
        _profile: &Profile,
        _data: &mut ContextData,
        _args: &BackendArguments
    ) -> Result<()> {
        Ok(())
    }

    fn render(
        &self,
        profile: &Profile,
        data: &ContextData
    ) -> Result<BackendArguments>;

}