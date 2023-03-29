use std::borrow::Cow;
use std::collections::{BinaryHeap, HashMap};

use anyhow::Result;
use serde::Serialize;

use crate::utils::{uri, xml};
use crate::BackendWWW;
use libfrt::entries::game::Game;
use libfrt::entries::media::{Image, ImageSource};

#[derive(Serialize, Debug)]
pub struct HtmlText {
    pub html: String,
    pub plain: String,
}

impl From<String> for HtmlText {
    fn from(s: String) -> Self {
        Self {
            html: xml::escape_str(s.as_str()).replace("\n", "<br />"),
            plain: s,
        }
    }
}

impl HtmlText {}

#[derive(PartialEq, Eq)]
pub enum HtmlImageMIME {
    ImageJpeg,
    ImagePng,
    ImageGif,
    ImageWebp,
    ImageApng,
    ImageAvif,
}

impl HtmlImageMIME {
    pub fn from_suffix<S>(suffix: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        match suffix.as_ref() {
            "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" => Some(Self::ImageJpeg),
            "png" => Some(Self::ImagePng),
            "gif" => Some(Self::ImageGif),
            "webp" => Some(Self::ImageWebp),
            "apng" => Some(Self::ImageApng),
            "avif" => Some(Self::ImageAvif),
            _ => None,
        }
    }

    pub fn from_src<S>(src: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let src = src.as_ref();

        match src.rfind('.') {
            Some(p) => Self::from_suffix(&src[p + 1..]),
            None => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            HtmlImageMIME::ImageJpeg => "image/jpeg",
            HtmlImageMIME::ImagePng => "image/pnf",
            HtmlImageMIME::ImageGif => "image/gif",
            HtmlImageMIME::ImageWebp => "image/webp",
            HtmlImageMIME::ImageApng => "image/apng",
            HtmlImageMIME::ImageAvif => "image/avif",
        }
    }

    pub fn priority(&self) -> u16 {
        match self {
            HtmlImageMIME::ImageJpeg => 20,
            HtmlImageMIME::ImagePng => 20,
            HtmlImageMIME::ImageGif => 10,
            HtmlImageMIME::ImageWebp => 30,
            HtmlImageMIME::ImageApng => 40,
            HtmlImageMIME::ImageAvif => 40,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct HtmlImageSrc {
    remote: bool,
    src: String,
    //size_hint: String,
}

impl HtmlImageSrc {
    pub fn to_str<'a, S>(&'a self, rr: S) -> Cow<'a, str>
    where
        S: AsRef<str>,
    {
        if self.remote {
            Cow::Borrowed(self.src.as_str())
        } else {
            Cow::Owned(format!("{}/{}", rr.as_ref(), self.src))
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct HtmlImageCondition {
    // currently only support one source
    pub srcset: HtmlImageSrc,
    pub mime: HtmlImageMIME,
}

impl Ord for HtmlImageCondition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mime.priority().cmp(&other.mime.priority())
    }
}

impl PartialOrd for HtmlImageCondition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct HtmlImage {
    sources: BinaryHeap<HtmlImageCondition>,
    size: Option<(usize, usize)>,
    captain: Option<String>,
    extra_queries: HashMap<String, String>,
}

impl HtmlImage {
    pub fn init_base_from_image(image: &Image) -> Self {
        Self {
            sources: BinaryHeap::default(),
            size: image.size.clone(),
            captain: image.captain.clone(),
            extra_queries: HashMap::default(),
        }
    }

    pub fn add_source_simple(
        &mut self,
        uri: String,
        remote: bool,
        mime: Option<HtmlImageMIME>,
    ) -> Result<()> {
        let mime = match mime {
            Some(m) => m,
            None => HtmlImageMIME::from_src(uri.as_str()).ok_or_else(|| {
                libfrt::err!(
                    InvalidArgument,
                    "Can not determine MIME for {}",
                    uri.as_str()
                )
            })?,
        };

        self.sources.push(HtmlImageCondition {
            srcset: HtmlImageSrc {
                remote: remote,
                src: uri,
            },
            mime: mime,
        });

        Ok(())
    }

    pub fn html<S, D>(&self, rr: S, node_classes: D, alt: Option<&str>) -> Result<String>
    where
        S: AsRef<str>,
        D: AsRef<str>,
    {
        let mut node = format!(r#"class="{}" "#, xml::escape_str(node_classes));

        if let Some((w, h)) = self.size {
            node.push_str(format!(r#"width="{}" height="{}" "#, w, h).as_str())
        }

        if let Some(alt) = match alt {
            Some(alt) => Some(alt),
            None => match self.captain.as_ref() {
                Some(alt) => Some(alt.as_str()),
                None => None,
            },
        } {
            node.push_str(format!(r#"alt="{}" "#, xml::escape_str(alt)).as_str())
        }

        let mut query_str = String::new();

        for (k, v) in self.extra_queries.iter() {
            if query_str.is_empty() {
                query_str = format!(r#"?{}={}"#, uri::encode_rfc3986(k), uri::encode_rfc3986(v));
            } else {
                query_str.push_str(
                    format!(r#"&{}={}"#, uri::encode_rfc3986(k), uri::encode_rfc3986(v)).as_str(),
                );
            }
        }

        let mut result = String::from("<picture>");

        let mut fb_cond = None;

        for cond in self.sources.iter() {
            result.push_str(
                format!(
                    r#"<source srcset="{}{}" type="{}">"#,
                    cond.srcset.to_str(rr.as_ref()),
                    query_str,
                    cond.mime.as_str()
                )
                .as_str(),
            );

            fb_cond = Some(cond);
        }

        match fb_cond {
            Some(fb_cond) => {
                result.push_str(
                    format!(
                        r#"<img {}src="{}{}" loading="lazy"></picture>"#,
                        node,
                        fb_cond.srcset.to_str(rr.as_ref()),
                        query_str
                    )
                    .as_str(),
                );
            }
            None => libfrt::bail!(InvalidArgument, "There isn't any candidates for image"),
        }

        Ok(result)
    }
}

pub struct HtmlMedia {
    pub html: String,
}

impl HtmlMedia {
    pub fn from_image(game: &Game, backend: &BackendWWW, image: &Image) -> Result<HtmlMedia> {
        match &image.source {
            ImageSource::LocalShared(_) => todo!(),
            ImageSource::Bundled(file_name) => {}
            ImageSource::Remote(uri) => {}
        }

        todo!("")
    }
}
