use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct ProfileWWW {
    pub path_templates: Vec<String>,
    pub path_static_layers: Vec<String>,
    pub path_stylesheets: Vec<String>,
    pub path_icon: Vec<String>,
}

impl Default for ProfileWWW {
    fn default() -> Self {
        Self {
            path_templates: vec![String::from("www/templates/**/*")],
            path_static_layers: vec![String::from("www/root")],
            path_stylesheets: vec![String::from("www/styles")],
            path_icon: vec![String::from("www/icons/build")],
        }
    }
}

impl ProfileWWW {
    pub fn from_value(value: toml::Value) -> Result<Self> {
        Ok(ProfileWWW::deserialize(value)?)
    }
}