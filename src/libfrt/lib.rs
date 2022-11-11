pub mod entries;
pub mod profile;
pub mod i18n;
pub mod utils;
pub mod error;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

use entries::{game::Game, author::Author};
use profile::Profile;
use error::{Error, ErrorKind};

#[derive(Default)]
pub struct Context {
    pub profile: Profile,

    pub(crate) authors: HashMap<String, Author>,
    pub(crate) games: HashMap<String, Game>,
}

impl Context {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile: profile,

            authors: HashMap::new(),
            games: HashMap::new(),
        }
    }

    pub fn build_game(&self, path: &Path) -> Result<Game> {
        let id = path.file_name()
            .ok_or_else(|| Error::new(ErrorKind::Other, "Can not get path name"))?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "Can not parse path name"))?
            .to_owned();

        let path_game_yaml = path.join("game.yaml");

        if !path_game_yaml.exists() {
            return Err(Error::new(ErrorKind::InvalidBundle, "Can not found game.yaml").into())
        }

        let raw_game = serde_yaml::from_str(&std::fs::read_to_string(path_game_yaml)?)?;

        let game = Game::build(self, id, raw_game)?;
        
        Ok(game)
    }

    pub fn load_game(&mut self, path: &Path) -> Result<()> {
        let game = self.build_game(path)?;
        self.games.insert(game.id.to_owned(), game);
        Ok(())
    }

    pub fn load_games(&mut self) -> Result<()> {
        let paths = fs::read_dir(&self.profile.path_games)?;
        for path in paths {
            let path = path?.path();
            self.load_game(&path)?;
        }
        Ok(())
    }

    pub fn build_author(&self, path: &Path) -> Result<Author> {
        todo!()
    }

    pub fn load_author(&mut self, path: &Path) -> Result<()> {
        Ok(())
    }

    pub fn load_authors(&mut self) -> Result<()> {
        let paths = fs::read_dir(&self.profile.path_authors)?;
        for path in paths {
            let path = path?.path();
            if path.ends_with(".yaml") {
                self.load_author(&path)?;
            }
        }
        Ok(())
    }

    pub fn full_init(&mut self) -> Result<()> {
        self.load_authors()?;
        self.load_games()?;
        Ok(())
    }
}