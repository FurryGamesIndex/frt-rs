
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Profile {
    authority_prefix: String,
    ui_config: String,
    stock_config: String
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            authority_prefix: String::from(""),
            ui_config: String::from("ui.toml"),
            stock_config: Path::new("frt").join("stock.toml").to_str().unwrap().to_string()
        }
    }
}

impl Profile {
    pub fn from_config(fpath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer = std::fs::read_to_string(fpath)?;
        Ok(toml::from_str(&buffer)?)
    }
}