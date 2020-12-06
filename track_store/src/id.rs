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
