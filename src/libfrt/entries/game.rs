use super::game_raw::RawGame;
use crate::Context;

pub struct Game {
    pub id: String,
}

impl Game {
    pub fn build(context: &Context, id: String, raw_game: RawGame) -> Self {
        Self {
            id: id
        }
    }
}