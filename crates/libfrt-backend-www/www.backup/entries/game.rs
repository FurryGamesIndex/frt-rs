use std::{path::PathBuf, collections::HashMap};

use anyhow::Result;
use serde::Serialize;

use crate::{entries::game::{Game, Description}, i18n::LangId};
use super::super::BackendWWW;
use super::common::HtmlText;

#[derive(Serialize, Debug)]
pub struct InnerGameWWW {
    pub name: HtmlText,
    pub description: HtmlText,
    pub brief_description: HtmlText,

    pub id: String,
    pub bundle_path: PathBuf,
}

#[derive(Serialize, Debug)]
pub struct GameWWW(pub String, pub HashMap<LangId, InnerGameWWW>);

impl GameWWW {
    pub fn from_game(game: &Game, backend: &BackendWWW) -> Result<GameWWW> {
        let mut game_l10ns = HashMap::new();

        for lang in backend.langs.iter() {
            let description = if game.l10n.contains_key(lang) {
                game.l10n.get(lang)
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap_or(&game.description)
            } else {
                &game.description
            };

            let description = match description {
                Description::Plain(s) => {
                    HtmlText::from(s.to_owned())
                },
                Description::Markdown(s) => {
                    HtmlText::from(format!("TODO: markdown is not supported now!\n\n{}", s))
                },
            };

            let brief_description = if game.l10n.contains_key(lang) {
                game.l10n.get(lang)
                    .unwrap()
                    .brief_description
                    .as_ref()
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
                    
                },
            };

            let game = InnerGameWWW {
                name: if game.l10n.contains_key(lang) {
                    game.l10n.get(lang)
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap_or(&game.name)
                } else {
                    &game.name
                }.to_owned().into(),
                description: description,
                brief_description: brief_description.into(),

                id: game.id.clone(),
                bundle_path: game.bundle_path.clone(),
            };

            game_l10ns.insert(lang.to_owned(), game);
        }

        Ok(GameWWW(game.id.to_owned(), game_l10ns))
    }
}