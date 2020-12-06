#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// The path of a track.
///
/// The [`TrackPath`](crate::TrackPath) is created from a [`String`](std::string::String) and contains the path to the track.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TrackPath(String);

impl From<String> for TrackPath {
    fn from(s: String) -> Self {
        TrackPath(s)
    }
}
