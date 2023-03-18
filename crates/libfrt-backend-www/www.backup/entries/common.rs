use anyhow::Result;
use serde::Serialize;

use crate::entries::game::Game;
use crate::backend::www::BackendWWW;
use crate::entries::media::{Image, ImageSource};

#[derive(Serialize, Debug)]
pub struct HtmlText {
    pub html: String,
    pub plain: String,
}

impl<> From<String> for HtmlText {
    fn from(s: String) -> Self {
        Self {
            html: tera::escape_html(s.as_str()).replace("\n", "<br />"),
            plain: s
        }
    }
}

impl HtmlText {
    
}

pub struct HtmlImageCondition {
    pub uri: String,
    pub mime: String,
}

pub struct HtmlImage {
    pub sources: Vec<HtmlImageCondition>,
    pub fb_uri: String,
    pub size: Option<(u32, u32)>,
    //pub extra_queries: HashMap<String, String>,
}

pub struct HtmlMedia {
    pub html: String,
}

impl HtmlMedia {
    pub fn from_image(game: &Game, backend: &BackendWWW, image: &Image) -> Result<HtmlMedia> {
        match &image.source {
            ImageSource::LocalShared(_) => todo!(),
            ImageSource::Bundled(file_name) => {
                
            },
            ImageSource::Remote(uri) => {

            },
        }

        todo!("")

    }
}