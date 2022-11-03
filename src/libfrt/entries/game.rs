use super::game_raw::RawGame;

pub struct Game {
    
}

impl Game {
    pub fn from_yaml(yaml_str: &str) -> Self {
        let raw_game: RawGame = serde_yaml::from_str(yaml_str).unwrap();


        Self {

        }
    }
}