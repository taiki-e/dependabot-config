use std::{fmt, io};

/// An error that occurred during parsing the Dependabot configuration.
#[derive(Debug)]
pub struct Error(ErrorKind);

// Hiding error variants from a library's public error type to prevent
// dependency updates from becoming breaking changes.
// We can add `is_*` methods that indicate the kind of error if needed, but
// don't expose dependencies' types directly in the public API.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    /// An error that occurred during parsing the configuration.
    Yaml(serde_yaml::Error),
    /// An error that occurred during converting different versions of the configuration.
    Convert(String),
}

impl Error {
    pub(crate) fn new(e: impl Into<ErrorKind>) -> Self {
        Self(e.into())
    }

    pub(crate) fn convert(e: impl Into<String>) -> Self {
        Self(ErrorKind::Convert(e.into()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorKind::Yaml(e) => fmt::Display::fmt(e, f),
            ErrorKind::Convert(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            ErrorKind::Yaml(e) => Some(e),
            ErrorKind::Convert(_) => None,
        }
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        match e.0 {
            ErrorKind::Yaml(e) => Self::new(io::ErrorKind::InvalidData, e),
            // TODO: Use ErrorKind::Unsupported once it stable: https://github.com/rust-lang/rust/pull/78880
            ErrorKind::Convert(e) => Self::new(io::ErrorKind::Other, e),
        }
    }
}

impl From<serde_yaml::Error> for ErrorKind {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e)
    }
}

// Note: These implementations are intentionally not-exist to prevent dependency
// updates from becoming breaking changes.
// impl From<serde_yaml::Error> for Error
