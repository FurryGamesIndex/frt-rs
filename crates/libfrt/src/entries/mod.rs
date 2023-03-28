use std::path::PathBuf;

pub mod link;
pub mod media;

pub mod game;
pub mod author;

pub(crate) mod raw;

pub trait Bundle {
    fn path(&self) -> &PathBuf;
    fn kind(&self) -> &'static str;
    fn id(&self) -> &str;
}