use super::raw::RawScreenshotItem;

#[derive(Debug)]
pub enum ImageSource {
    LocalShared(String),
    Bundled(String),
    Remote(String),
}

#[derive(Debug)]
pub struct VideoSource {
    pub mime: String,
    pub uri: String,
}

#[derive(Debug)]
pub struct Image {
    source: ImageSource,
    captain: String,

    /// Size of the image, width and height.
    /// Maybe unavailable for remote images.
    size: Option<(u32, u32)>,

    /// Timestamp for last modified of this image.
    /// Maybe unavailable for remote images.
    mtime: Option<i64>,
}

#[derive(Debug)]
pub enum Media {
    Image(Image),
    Youtube(String),
    Video {
        sources: Vec<VideoSource>,
    }
}

impl<> From<RawScreenshotItem> for Media {
    fn from(_: RawScreenshotItem) -> Self {
        error!("impl<> From<RawScreenshotItem> for Media: fn from(): STUB!"); /* TODO */
        Self::Image(Image {
            source: ImageSource::Remote("http://example.com".to_string()),
            captain: "STUB".to_string(),
            size: None,
            mtime: None
        })
    }
}

impl<> From<String> for Image {
    fn from(_: String) -> Self {
        error!("impl<> From<String> for Image: fn from(): STUB!"); /* TODO */
        Image {
            source: ImageSource::Remote("http://example.com".to_string()),
            captain: "STUB".to_string(),
            size: None,
            mtime: None
        }
    }
}