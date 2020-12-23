#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use std::path::Path;

/// The track (i.e. its path)
///
/// The [`Track`](crate::Track) is created from a [`String`](std::string::String) or a [`&str`](std::str) and contains the path to the track.
#[derive(Debug, PartialEq, Clone)]
pub struct Track(String);

impl From<String> for Track {
    fn from(path: String) -> Self {
        Track(path)
    }
}

impl From<&str> for Track {
    fn from(path: &str) -> Self {
        Track(path.into())
    }
}

impl AsRef<Path> for Track {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
