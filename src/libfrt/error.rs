use core::fmt;
use std::fmt::Display;

/// Errors that can occur in FRT.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

/// Errors that can occur in FRT.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    /// Any other kind of errors not listed.
    Other,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Other => write!(f, "{}", "Other: ")?
        };
        write!(f, "{}", self.message)?;
        Ok(())
    }
}