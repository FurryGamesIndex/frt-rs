use std::collections::HashMap;

use anyhow::Result;

use super::raw::{RawGame, RawLinkItem};
use super::link::Link;
use crate::ContextData;
use crate::error::{Error, ErrorKind};
use crate::i18n::LangId;

#[derive(Debug)]
pub enum Description {
    Plain(String),
    Markdown(String),
}

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub description: Description,
    pub brief_description: Option<String>,
    pub links: Vec<Link>,

    pub l10n: HashMap<LangId, GameL10n>,
}

#[derive(Debug)]
pub struct GameL10n {
    pub name: Option<String>,
    pub description: Option<Description>,
}

impl Game {
    pub fn build(data: &ContextData, id: String, raw_game: RawGame) -> Result<Self> {

        let description = match raw_game.description_format {
            Some(format) => {
                match format.as_str() {
                    "plain" => Ok(Description::Plain(raw_game.description)),
                    "markdown" => Ok(Description::Markdown(raw_game.description)),
                    _ => Err(Error::new(ErrorKind::InvalidArgument, 
                            format!("Unknown description format: {}", format).as_str()))
                }
            },
            None => Ok(Description::Plain(raw_game.description)),
        }?;

        let links: Result<Vec<_>> = raw_game.links.into_iter()
            .map(|raw_link| data.link_rules.build_link(raw_link))
            .collect();
        let links = links?;

        Ok(Self {
            id: id,
            name: raw_game.name,
            description: description,
            brief_description: raw_game.brief_description,
            links: links,

            l10n: HashMap::new(),
        })
    }
}