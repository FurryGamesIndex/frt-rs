use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::link::Link;
use super::media::{Image, Media};
use super::raw::RawGame;
use super::Bundle;
use crate::i18n::LangId;
use crate::ContextData;

#[derive(Debug)]
pub enum Description {
    Plain(String),
    Markdown(String),
}

#[derive(Debug)]
pub struct GameMedia {
    pub sensitive: bool,
    pub media: Media,
}

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub description: Description,
    pub brief_description: Option<String>,
    pub links: Vec<Link>,
    pub medias: Vec<GameMedia>,
    pub thumbnail: Image,

    pub l10n: HashMap<LangId, GameL10n>,

    pub bundle_path: PathBuf,
    //pub dirty: bool,
}

impl Bundle for Game {
    fn path(&self) -> &PathBuf {
        &self.bundle_path
    }

    fn kind(&self) -> &'static str {
        "game"
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Debug)]
pub struct GameL10n {
    pub name: Option<String>,
    pub description: Option<Description>,
    pub brief_description: Option<String>,
}

impl Game {
    pub fn build(
        data: &ContextData,
        id: String,
        raw_game: RawGame,
        bundle_path: PathBuf,
    ) -> Result<Self> {
        let description = match raw_game.description_format {
            Some(format) => match format.as_str() {
                "plain" => Ok(Description::Plain(raw_game.description)),
                "markdown" => Ok(Description::Markdown(raw_game.description)),
                _ => Err(crate::err!(
                    InvalidArgument,
                    "Unknown description format: {}",
                    format
                )),
            },
            None => Ok(Description::Plain(raw_game.description)),
        }?;

        let links: Result<Vec<_>> = raw_game
            .links
            .into_iter()
            .map(|raw_link| data.link_rules.build_link(raw_link))
            .collect();
        let links = links?;

        let mut medias = Vec::new();
        for ss in raw_game.screenshots.into_iter() {
            medias.push(GameMedia {
                sensitive: ss.is_sensitive(),
                media: Media::from_raw(ss, Some(&bundle_path))?,
            });
        }

        Ok(Self {
            id: id,
            name: raw_game.name,
            description: description,
            brief_description: raw_game.brief_description,
            links: links,
            medias: medias,
            thumbnail: Image::from_str(&raw_game.thumbnail, None, Some(&bundle_path))?,

            l10n: HashMap::new(),

            bundle_path: bundle_path,
            //dirty: true,
        })
    }

    pub fn from_bundle(data: &ContextData, path: &Path) -> Result<Game> {
        info!("Loading game bundle: {}", path.display());

        let id = path
            .file_name()
            .ok_or_else(|| crate::err!(Other, "Can not get path name"))?
            .to_str()
            .ok_or_else(|| crate::err!(Other, "Can not parse path name"))?
            .to_owned();

        let path_game_yaml = path.join("game.yaml");

        if !path_game_yaml.exists() {
            crate::bail!(InvalidBundle, "Can not found game.yaml")
        }

        let raw_game = serde_yaml::from_str(&std::fs::read_to_string(path_game_yaml)?)?;

        let game = Game::build(data, id, raw_game, std::fs::canonicalize(path)?)?;

        Ok(game)
    }
}
