use anyhow::Result;
use libfrt::{backend::Backend, profile::Profile};

#[cfg(feature = "backend-www")]
use libfrt_backend_www::BackendWWW;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CliBackend {
    #[cfg(feature = "backend-www")]
    WWW,
    #[cfg(feature = "backend-null")]
    Null,
}

impl CliBackend {
    pub fn new_backend(&self, profile: &mut Profile) -> Result<Option<Box<dyn Backend>>> {
        Ok(match self {
            CliBackend::WWW => {
                let v = profile.backends.remove("www");
                Some(Box::new(BackendWWW::new(v)?))
            }
            CliBackend::Null => None,
        })
    }
}
