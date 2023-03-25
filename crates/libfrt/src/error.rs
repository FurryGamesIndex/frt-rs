use core::fmt;
use std::fmt::Display;

/// Errors that can occur in FRT.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

/// Errors that can occur in FRT.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    /// Bundle invalid, missing required files. etc
    InvalidBundle,

    /// Invalid value format, unknown enum value, etc
    InvalidArgument,

    /// Template, rule, image or any dependent resource not found
    NotExist,

    /// External file not valid, breaked image format, etc
    InvalidFileOrData,

    /// Any other kind of errors not listed.
    Other,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::InvalidBundle => write!(f, "{}", "Bundle is invalid: ")?,
            ErrorKind::InvalidArgument => write!(f, "{}", "Invalid argument: ")?,
            ErrorKind::NotExist => write!(f, "{}", "No such resouce: ")?,
            ErrorKind::InvalidFileOrData => write!(f, "{}", "Invalid file or data: ")?,
            ErrorKind::Other => write!(f, "{}", "Other: ")?,
        };
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind: kind,
            message: message.to_owned(),
        }
    }
}

#[macro_export]
macro_rules! err {
    ($kind:ident, $msg:literal) => {
        $crate::error::Error::new(
            $crate::error::ErrorKind::$kind,
            $msg
        )
    };
    ($kind:ident, $fmt:expr, $($arg:tt)*) => {
        $crate::error::Error::new(
            $crate::error::ErrorKind::$kind,
            format!($fmt, $($arg)*).as_str()
        )
    };
}

#[macro_export]
macro_rules! bail {
    ($kind:ident, $msg:literal) => {
        return core::result::Result::Err(
            $crate::error::Error::new(
                $crate::error::ErrorKind::$kind,
                $msg
            )
            .into()
        )
    };
    ($kind:ident, $fmt:expr, $($arg:tt)*) => {
        return core::result::Result::Err(
            $crate::error::Error::new(
                $crate::error::ErrorKind::$kind,
                format!($fmt, $($arg)*).as_str()
            )
            .into()
        )
    };
}
