use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use serde::Serialize;

use crate::BackendWWW;

use super::common::{HtmlImage, HtmlText};
use libfrt::{
    entries::game::{Description, Game},
    i18n::LangId,
};

pub struct CookedGameL10n {
    pub name: HtmlText,
    pub description: HtmlText,
    pub brief_description: HtmlText,
}

pub struct CookedGameNonl10n {
    pub thumbnail: HtmlImage,
}

pub struct GameWWW {
    pub orig: Rc<Game>,
    pub cooked: HashMap<LangId, CookedGameL10n>,
    pub cooked_nonl10n: CookedGameNonl10n,
}

impl GameWWW {
    pub fn cook_game(game: Rc<Game>, backend: &BackendWWW) -> Result<GameWWW> {
        let mut cooked = HashMap::new();

        for lang in backend.langs.iter() {
            let description = match game.l10n.get(lang) {
                Some(gl10n) => gl10n.description.as_ref().unwrap_or(&game.description),
                None => &game.description,
            };

            let description = match description {
                Description::Plain(s) => HtmlText::from(s.to_owned()),
                Description::Markdown(s) => {
                    HtmlText::from(format!("TODO: markdown is not supported now!\n\n{}", s))
                }
            };

            let brief_description = match game.l10n.get(lang) {
                Some(gl10n) => gl10n.brief_description.as_ref(),
                None => game.brief_description.as_ref(),
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

            cooked.insert(
                lang.to_owned(),
                CookedGameL10n {
                    name: match game.l10n.get(lang) {
                        Some(gl10n) => gl10n.name.as_ref().unwrap_or(&game.name),
                        None => &game.name,
                    }
                    .to_owned()
                    .into(),
                    description: description,
                    brief_description: brief_description.into(),
                },
            );
        }

        Ok(GameWWW {
            orig: game,
            cooked,
            cooked_nonl10n: CookedGameNonl10n { thumbnail: todo!() },
        })
    }
}
