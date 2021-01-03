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

impl AsRef<str> for Track {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<Path> for Track {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_as_ref() {
        let track = Track::from("track");
        let as_ref: &str = track.as_ref();
        assert_eq!("track", as_ref);
    }

    #[test]
    fn test_display() {
        let track = Track::from("track");
        let formatted_track = format!("{}", track);
        assert_eq!("track".to_string(), formatted_track);
    }
}
