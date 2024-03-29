use anyhow::Result;
use std::path::PathBuf;

use crate::utils::is_remote::is_remote;

use super::raw::{RawScreenshotItem, RawVideoSourceItem};

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

impl From<RawVideoSourceItem> for VideoSource {
    fn from(value: RawVideoSourceItem) -> Self {
        Self {
            mime: value.mime,
            uri: value.uri,
        }
    }
}

#[derive(Debug)]
pub enum Media {
    Image(Image),
    Youtube(String),
    Video { sources: Vec<VideoSource> },
    HBox(Vec<Image>),
}

impl From<Image> for Media {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl Media {
    pub fn from_raw(raw: RawScreenshotItem, bundle_path: Option<&PathBuf>) -> Result<Self> {
        match raw {
            RawScreenshotItem::SimpleImage(uri) | RawScreenshotItem::Image { uri, .. } => {
                Ok(Self::Image(Image::from_str(uri, None, bundle_path)?))
            }
            RawScreenshotItem::Youtube { youtube } => Ok(Self::Youtube(youtube)),
            RawScreenshotItem::Video { video, .. } => Ok(Self::Video {
                sources: video.into_iter().map(|rvs| rvs.into()).collect(),
            }),
            RawScreenshotItem::HBox { hbox, .. } => {
                let mut result = Vec::new();
                for image in hbox.into_iter() {
                    result.push(Image::from_str(image, None, bundle_path)?);
                }
                Ok(Self::HBox(result))
            }
        }
    }
}
