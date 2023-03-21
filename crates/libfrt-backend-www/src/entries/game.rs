use std::{collections::HashMap, path::PathBuf, rc::Rc};

use anyhow::Result;
use serde::Serialize;

use crate::BackendWWW;
use super::common::HtmlText;
use libfrt::{
    entries::game::{Description, Game},
    i18n::LangId,
};

#[derive(Serialize, Debug)]
pub struct CookedGame {
    pub name: HtmlText,
    pub description: HtmlText,
    pub brief_description: HtmlText,
}

pub struct GameWWW {
    pub game: Rc<Game>,
    pub cooked: HashMap<LangId, CookedGame>,
}

impl GameWWW {
    pub fn cook_game(game: Rc<Game>, backend: &BackendWWW) -> Result<GameWWW> {
        let mut game_l10ns = HashMap::new();

        for lang in backend.langs.iter() {
            let description = if game.l10n.contains_key(lang) {
                game.l10n
                    .get(lang)
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap_or(&game.description)
            } else {
                &game.description
            };

            let description = match description {
                Description::Plain(s) => HtmlText::from(s.to_owned()),
                Description::Markdown(s) => {
                    HtmlText::from(format!("TODO: markdown is not supported now!\n\n{}", s))
                }
            };

            let brief_description = if game.l10n.contains_key(lang) {
                game.l10n.get(lang).unwrap().brief_description.as_ref()
            } else {
                game.brief_description.as_ref()
            };

            let brief_description = match brief_description {
                Some(d) => d.clone(),
                None => {
                    let vec: Vec<_> = description.plain.chars().collect();
                    if vec.len() > 480 {
                        let s: String = vec.into_iter().take(480).collect();
                        s + "..."
                    } else {
                        description.plain.clone()
                    }
                }
            };

            game_l10ns.insert(
                lang.to_owned(),
                CookedGame {
                    name: if game.l10n.contains_key(lang) {
                        game.l10n
                            .get(lang)
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap_or(&game.name)
                    } else {
                        &game.name
                    }
                    .to_owned()
                    .into(),
                    description: description,
                    brief_description: brief_description.into(),
                },
            );
        }

        Ok(GameWWW {
            game,
            cooked: game_l10ns,
        })
    }
}
