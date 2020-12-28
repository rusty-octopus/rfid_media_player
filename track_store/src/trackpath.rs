#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The path of a track.
///
/// The [`TrackPath`](crate::TrackPath) is created from a [`String`](std::string::String) or a [`&str`](std::str) and contains the path to the track.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TrackPath(String);

impl From<String> for TrackPath {
    fn from(s: String) -> Self {
        TrackPath(s)
    }
}

impl From<&str> for TrackPath {
    fn from(path: &str) -> Self {
        TrackPath(path.into())
    }
}

impl AsRef<str> for TrackPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for TrackPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:}", self)
    }
}
