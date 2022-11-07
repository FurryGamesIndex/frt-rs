use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawAuthorItem {
    pub name: String,
    pub role: Vec<String>,
    #[serde(default)]
    pub standalone: bool,
}


#[derive(Deserialize, Debug)]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum RawScreenshotItem {
    SimpleImage (String),
    Image {
        #[serde(default)]
        sensitive: bool,
        uri: String,
    },
    Youtube {
        youtube: String,
    },
    Video {
        video: Vec<RawVideoSourceItem>,
    }

}

#[derive(Deserialize, Debug)]
pub struct RawGame {
    pub name: String,

    pub description: String,

    #[serde(rename = "description-format")]
    pub description_format: Option<String>,

    #[serde(rename = "brief-description")]
    pub brief_description: Option<String>,

    pub thumbnail: String,

    #[serde(default)]
    pub authors: Vec<RawAuthorItem>,

    #[serde(default)]
    pub links: Vec<RawLinkItem>,

    #[serde(default)]
    pub screenshots: Vec<RawScreenshotItem>
}