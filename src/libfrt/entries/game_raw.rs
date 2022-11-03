use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawAuthorItem {
    pub name: String,
    pub role: Vec<String>,
    #[serde(default)]
    pub standalone: bool,
}


#[derive(Deserialize, Debug)]
pub enum RawLinkItem {
    Custom {
        name: String,
        uri: String,
    },
    Auto(String)
}

#[derive(Deserialize, Debug)]
pub struct RawVideoSourceItem {
    pub mime: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub enum RawScreenshotItem {
    Image (String),
    Youtube {
        #[serde(rename = "type")]
        kind: String,
        id: String,
    },
    Video {
        #[serde(rename = "type")]
        kind: String,
        src: Vec<RawVideoSourceItem>,
    }

}

#[derive(Deserialize, Debug)]
pub struct RawGame {
    pub name: String,

    pub description: String,

    #[serde(rename = "description-format")]
    pub description_format: String,

    #[serde(rename = "brief-description")]
    pub brief_description: Option<String>,

    pub thumbnail: String,

    pub authors: Vec<RawAuthorItem>,

    pub links: Vec<RawLinkItem>,

    pub screenshots: Vec<RawScreenshotItem>
}