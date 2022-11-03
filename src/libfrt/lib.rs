
pub mod entries;
pub mod profile;
pub mod utils;
pub mod error;

use std::fs;
use std::path::Path;
use anyhow::Result;

use entries::{game::Game, author::Author};
use profile::Profile;
use error::Error;

#[derive(Default)]
pub struct Context {
    pub profile: Profile
}

impl Context {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile: profile
        }
    }

    pub fn load_game(&self, path: &str) -> Result<Game> {
        Ok(Game{})
    }

    pub fn load_games(&self) -> Result<()> {
        let paths = fs::read_dir(&self.profile.path_games)?;
        for path in paths {
            let path = path?.path().to_str().unwrap().to_string();
            self.load_game(&path)?;
        }
        Ok(())
    }

    pub fn full_init(&self) -> Result<()> {
        self.load_games()?;
        Ok(())
    }
}