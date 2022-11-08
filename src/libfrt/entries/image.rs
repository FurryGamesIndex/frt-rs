#[derive(Debug)]
pub enum ImageSource {
    LocalShared(String),
    Bundled(String),
    Remote(String),
}

#[derive(Debug)]
pub struct Image {
    pub source: ImageSource,
    pub captain: String,

    /// Size of the image, width and height.
    /// Maybe unavailable for remote images.
    pub size: Option<(u32, u32)>,

    /// Timestamp for last modified of this image.
    /// Maybe unavailable for remote images.
    pub mtime: Option<i64>,
}
