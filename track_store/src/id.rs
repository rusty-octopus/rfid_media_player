#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
/// The Id of a track.
///
/// In order to receive a [`TrackPath`](crate::TrackPath) from the [`TrackStore`](crate::TrackStore),
/// one must must create an Id from a [`String`](std::string::String).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Id(String);

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id(s)
    }
}

impl From<&str> for Id {
    fn from(path: &str) -> Self {
        Id(path.into())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:}", self)
    }
}
