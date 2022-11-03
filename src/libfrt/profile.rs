use serde::Deserialize;
use std::path::Path;

use anyhow::Result;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Profile {
    pub authority_prefix: String,
    pub ui_config: String,
    pub stock_config: String,
    pub path_games: String,
    pub path_authors: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            authority_prefix: String::from(""),
            ui_config: String::from("ui.toml"),
            stock_config: Path::new("frt").join("stock.toml").to_str().unwrap().to_string(),
            path_games: String::from("games"),
            path_authors: String::from("authors"),
        }
    }
}

impl Profile {
    pub fn from_config(fpath: &str) -> Result<Self> {
        let buffer = std::fs::read_to_string(fpath)?;
        Ok(toml::from_str(&buffer)?)
    }
}