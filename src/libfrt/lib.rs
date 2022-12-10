pub mod entries;
pub mod profile;
pub mod backend;
pub mod i18n;
pub mod utils;
pub mod error;

#[macro_use] extern crate log;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use anyhow::Result;

use entries::raw::RawStockConfig;
use entries::{game::Game, author::Author};
use entries::link::LinkRuleManager;
use backend::{Backend, BackendArguments, www::BackendWWW};
use i18n::LangId;
use profile::Profile;
use error::{Error, ErrorKind};

#[derive(Default)]
pub struct ContextData {
    pub authors: HashMap<String, Author>,
    pub games: HashMap<String, Game>,

    pub link_rules: LinkRuleManager,

    pub ui: HashMap<LangId, toml::Value>,
}

impl ContextData {
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
        info!("Loading {}", path.display());
        let game = self.build_game(path)?;
        self.games.insert(game.id.to_owned(), game);
        Ok(())
    }

    pub fn build_author(&self, path: &Path) -> Result<Author> {
        todo!()
    }

    pub fn load_author(&mut self, path: &Path) -> Result<()> {
        Ok(())
    }

    pub fn load_stock(&mut self, file: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file)?;
        let mut stock_config: RawStockConfig = toml::from_str(content.as_str())?;

        for (rule_name, mut rule) in stock_config.link.drain() {
            rule.name = rule_name;
            self.link_rules.add_rule(rule)?;
        }

        Ok(())
    }

    pub fn load_ui(&mut self, file: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file)?;
        self.ui.entry(LangId::default()).or_insert(toml::from_str("")?);
        utils::toml::merge(self.ui.get_mut(&LangId::default()).unwrap(), toml::from_str(content.as_str())?);
        Ok(())
    }

    pub fn post_load_ui(&mut self) -> Result<()> {
        let mut orig_ui = self.ui.remove(&LangId::default()).unwrap();

        match &mut orig_ui {
            toml::Value::Table(table) => {
                let low_ui = table.remove("_").ok_or_else(||
                    Error::new(ErrorKind::InvalidArgument, 
                        "The '_' entry was not found in ui config"))?;

                for (lang, value) in table.iter() {
                    let mut v = low_ui.clone();

                    // unexcepted memcpy, but seems no `drain()` for toml::Map
                    utils::toml::merge(&mut v, value.clone());

                    self.ui.insert(lang.as_str().into(), v);
                }

                self.ui.entry(LangId::default()).or_insert(low_ui);
            },
            _ => return Err(Error::new(ErrorKind::InvalidArgument, 
                "Invalid ui config format").into()),
        }

        Ok(())
    }
}

pub struct Context {
    pub profile: Profile,

    pub(crate) backend: Option<Rc<dyn Backend>>,

    pub(crate) data: ContextData,
}

impl Context {
    pub fn new(mut profile: Profile) -> Result<Self> {
        let backend_profile_value = profile.backends.remove("www");
        profile.backends.clear();

        Ok(Self {
            profile: profile,

            // Currently we are only supporting the `www` backend.
            // So, we hard-code the backend here.
            backend: Some(Rc::from(BackendWWW::new(backend_profile_value)?)),

            data: ContextData::default(),
        })
    }

    pub fn load_games(&mut self) -> Result<()> {
        for i in self.profile.path_games.iter() {
            info!("Loading game dir: {i}");
            let paths = fs::read_dir(i)?;
            for path in paths {
                let path = path?.path();
                if path.is_dir() {
                    self.data.load_game(&path)?;
                }
            }
        }

        info!("{} games Loaded", self.data.games.len());

        Ok(())
    }

    pub fn load_authors(&mut self) -> Result<()> {
        for i in self.profile.path_authors.iter() {
            info!("Loading author dir: {i}");
            let paths = fs::read_dir(i)?;
            for path in paths {
                let path = path?.path();
                if path.ends_with(".yaml") {
                    self.data.load_author(&path)?;
                }
            }
        }

        Ok(())
    }

    pub fn load_config(&mut self) -> Result<()> {
        for i in &self.profile.stock_config {
            info!("Loading stock config '{i}'");
            self.data.load_stock(Path::new(i))?;
        }

        for i in &self.profile.ui_config {
            info!("Loading ui config '{i}'");
            self.data.load_ui(Path::new(i))?;
        }

        self.data.post_load_ui()?;

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        info!("Context initializing");
        self.load_config()?;
        Ok(())
    }

    pub fn full_init(&mut self) -> Result<()> {
        self.init()?;
        self.load_authors()?;
        self.load_games()?;
        Ok(())
    }

    pub fn invoke_backend(&self, args: BackendArguments) -> Result<BackendArguments> {
        self.backend.as_ref().unwrap().render(&self.profile, &self.data, args)
    }
}