#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use media_player;
use rfid_reader;
use track_store;

/// RFID Media Player Errors
///
/// Defines all runtime errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Media player error.
    ///
    /// Returned whenever an error with the media player occurred.
    MediaPlayerError(String),
    /// RFID reader error.
    ///
    /// Returned whenever an error with the RFID reader occurred.
    RfidReaderError(String),
    /// Track store error.
    ///
    /// Returned whenever an error with the track store occurred.
    TrackStoreError(String),
}

macro_rules! implement_from_error_trait {
    ($error_type:tt, $other_error_type:ty, $error_variant:ident) => {
        impl From<$other_error_type> for $error_type {
            fn from(error: $other_error_type) -> Self {
                $error_type::$error_variant(format!("{}", error))
            }
        }
    };
}

implement_from_error_trait!(Error, media_player::Error, MediaPlayerError);
implement_from_error_trait!(Error, rfid_reader::Error, RfidReaderError);
implement_from_error_trait!(Error, track_store::Error, TrackStoreError);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_from_media_player_error() {
        let media_player_error = media_player::Error::IoError("Test".to_string());
        let error = Error::from(media_player_error);
        assert_eq!(
            Error::MediaPlayerError("IoError(\"Test\")".to_string()),
            error
        );
    }

    #[test]
    fn test_from_rfid_reader_error() {
        let other_error = rfid_reader::Error::Timeout;
        let error = Error::from(other_error);
        assert_eq!(Error::RfidReaderError("Timeout".to_string()), error);
    }

    #[test]
    fn test_from_track_store_error() {
        let other_error = track_store::Error::LoadError("Test".to_string());
        let error = Error::from(other_error);
        assert_eq!(
            Error::TrackStoreError("LoadError(\"Test\")".to_string()),
            error
        );
    }

    #[test]
    fn test_display() {
        let other_error = rfid_reader::Error::Timeout;
        let error = Error::from(other_error);
        let formatted_error = format!("{:}", error);
        assert_eq!("RfidReaderError(\"Timeout\")", formatted_error);
    }
}
