use anyhow::Result;
use std::path::PathBuf;

use crate::utils::is_remote::is_remote;

use super::raw::RawScreenshotItem;

#[derive(Debug)]
pub enum ImageSource {
    LocalShared(String),
    Bundled(String),
    Remote(String),
}

#[derive(Debug)]
pub struct Image {
    pub source: ImageSource,
    pub captain: Option<String>,

    /// Size of the image, width and height.
    /// Maybe unavailable for remote images.
    pub size: Option<(usize, usize)>,

    /// Timestamp for last modified of this image.
    /// Maybe unavailable for remote images.
    pub mtime: Option<i64>,
}

impl Image {
    pub fn new_bundled(name: String, captain: Option<String>, file_path: &PathBuf) -> Result<Self> {
        let dim = imagesize::size(&file_path).map_err(|e| {
            crate::err!(
                InvalidFileOrData,
                "Can not read image properties for '{}': {:?}",
                file_path.display().to_string(),
                e
            )
        })?;

        Ok(Image {
            source: ImageSource::Bundled(name),
            captain,
            size: Some((dim.width, dim.height)),
            mtime: None,
        })
    }

    pub fn new_remote(name: String, captain: Option<String>) -> Self {
        Image {
            source: ImageSource::Remote(name),
            captain,
            size: None,
            mtime: None,
        }
    }

    pub fn from_str<S>(
        src: S,
        captain: Option<String>,
        bundle_path: Option<&PathBuf>,
    ) -> Result<Image>
    where
        S: AsRef<str>,
    {
        if is_remote(&src) {
            Ok(Self::new_remote(src.as_ref().to_owned(), captain))
        } else {
            match bundle_path {
                Some(bp) => {
                    let file_path = bp.join(src.as_ref());
                    if file_path.is_file() {
                        Image::new_bundled(src.as_ref().to_owned(), captain, &file_path)
                    } else {
                        crate::bail!(
                            NotExist,
                            "Image file not found: {}",
                            file_path.to_string_lossy()
                        )
                    }
                }
                None => crate::bail!(
                    InvalidArgument,
                    "Bundle path is required for non-remote images."
                ),
            }
        }
    }
}

#[derive(Debug)]
pub struct VideoSource {
    pub mime: String,
    pub uri: String,
}

#[derive(Debug)]
pub enum Media {
    Image(Image),
    Youtube(String),
    Video { sources: Vec<VideoSource> },
}

impl From<RawScreenshotItem> for Media {
    fn from(_: RawScreenshotItem) -> Self {
        error!("impl<> From<RawScreenshotItem> for Media: fn from(): STUB!"); /* TODO */
        Self::Image(Image {
            source: ImageSource::Remote("http://example.com".to_string()),
            captain: None,
            size: None,
            mtime: None,
        })
    }
}

impl From<Image> for Media {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}
