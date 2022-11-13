use std::{path::Path, collections::HashMap};

use serde::Deserialize;
use toml::Value;
use anyhow::Result;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Profile {
    pub authority_prefix: String,

    pub ui_config: Vec<String>,
    pub stock_config: Vec<String>,
    pub path_games: Vec<String>,
    pub path_authors: Vec<String>,

    pub backends: HashMap<String, Value>
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            authority_prefix: String::from(""),

            ui_config: vec![String::from("ui.toml")],
            stock_config: vec![String::from("stock.toml")],
            path_games: vec![String::from("games")],
            path_authors: vec![String::from("authors")],

            backends: HashMap::new(),
        }
    }
}

impl Profile {
    pub fn from_config(fpath: &str) -> Result<Self> {
        let buffer = std::fs::read_to_string(fpath)?;
        Ok(toml::from_str(&buffer)?)
    }
}