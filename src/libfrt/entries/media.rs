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
pub enum Media {
    Image {
        source: ImageSource,
        captain: String,
    
        /// Size of the image, width and height.
        /// Maybe unavailable for remote images.
        size: Option<(u32, u32)>,
    
        /// Timestamp for last modified of this image.
        /// Maybe unavailable for remote images.
        mtime: Option<i64>,
    },
    Youtube(String),
    Video {
        sources: Vec<VideoSource>,
    }
}

impl<> From<RawScreenshotItem> for Media {
    fn from(_: RawScreenshotItem) -> Self {
        todo!()
    }
}