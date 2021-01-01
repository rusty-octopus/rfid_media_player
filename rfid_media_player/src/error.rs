#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use media_player;
use rfid_reader;
use track_store;

#[derive(Debug, PartialEq)]
pub enum Error {
    MediaPlayerError(String),
    RfidReaderError(String),
    TrackStoreError(String),
    Unknown,
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
