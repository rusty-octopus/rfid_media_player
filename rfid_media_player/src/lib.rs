//! The library of the rfid_media_player application.
//!
//! The [`RfidMediaPlayer`](crate::RfidMediaPlayer) can be [`open`](crate::open)ed in order to run it in the `main.rs`.
//! It also helps creating an app with [crate::create_app](crate::create_app) to be used for the command line application.
//! Finally it defines [`Error`](crate::Error)s that can happen during open or running the application.
//!
//! This library is used internally by the rfid_media_player application.
//! It is implemented in order to test most of the logic of the main application.
//!
//! Example usage can be found in `main.rs`.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use media_player;
use rfid_reader::{ProductId, VendorId};
use track_store;

use std::time::Duration;

use log::error;

mod cli_app;
mod error;
mod rfidmediaplayer;

pub use cli_app::{
    convert_to_id, create_app, CONSOLE_LOG_ARGUMENT_NAME, LOG_LEVEL_ARGUMENT_NAME,
    PRODUCT_ID_ARGUMENT_NAME, TIMEOUT_ARGUMENT_NAME, TRACKS_FILE_ARGUMENT_NAME,
    VENDOR_ID_ARGUMENT_NAME,
};
pub use error::Error;
pub use rfidmediaplayer::RfidMediaPlayer;

/// Tries to open the [`RfidMediaPlayer`](crate::RfidMediaPlayer).
///
/// Returns an [`RfidMediaPlayer`](crate::RfidMediaPlayer) trait object.
///
/// Returns an [`Error`](crate::Error) whenever there is a problem with the rfid_reader, track_store or media_player.
pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
    yaml_string: &str,
) -> Result<impl RfidMediaPlayer, crate::Error> {
    let mut result = Err(Error::TrackStoreError(
        "Track store not loaded yet".to_string(),
    ));
    let result_track_store = track_store::load(yaml_string);
    if let Ok(track_store) = result_track_store {
        let result_rfid_reader = rfid_reader::open(vendor_id, product_id, timeout);
        if let Ok(rfid_reader) = result_rfid_reader {
            let result_media_player = media_player::open();
            if let Ok(media_player) = result_media_player {
                result = Ok(rfidmediaplayer::open(
                    media_player,
                    rfid_reader,
                    track_store,
                ));
            } else if let Err(media_player_error) = result_media_player {
                error!(
                    "Opening media player resulted in error: {}",
                    media_player_error
                );
                result = Err(crate::Error::from(media_player_error))
            }
        } else if let Err(rfid_reader_error) = result_rfid_reader {
            error!(
                "Opening rfid reader resulted in error: {}",
                rfid_reader_error
            );
            result = Err(crate::Error::from(rfid_reader_error))
        }
    } else if let Err(result_track_store_error) = result_track_store {
        error!(
            "Loading track store resulted in error: {}",
            result_track_store_error
        );
        result = Err(crate::Error::from(result_track_store_error))
    }

    result
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_yaml() {
        let result = open(
            VendorId::from(1),
            ProductId::from(1),
            Duration::from_millis(1000),
            "test",
        );
        if let Err(error) = result {
            assert_eq!(Error::TrackStoreError("LoadError(\"invalid type: string \\\"test\\\", expected a map at line 1 column 1\")".to_string()), error);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_wrong_device() {
        let result = open(
            VendorId::from(1),
            ProductId::from(1),
            Duration::from_millis(1000),
            "1234: path",
        );
        if let Err(error) = result {
            assert_eq!(
                Error::RfidReaderError("DeviceNotFound(VendorId(1), ProductId(1))".to_string()),
                error
            );
        } else {
            assert!(false);
        }
    }
}
