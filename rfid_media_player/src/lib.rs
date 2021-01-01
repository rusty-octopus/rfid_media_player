use media_player;
use rfid_reader::{ProductId, VendorId};
use track_store;

use std::time::Duration;

use log::error;

mod error;
mod rfidmediaplayer;

pub use error::Error;
pub use rfidmediaplayer::RfidMediaPlayer;

pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
    yaml_string: &str,
) -> Result<impl RfidMediaPlayer, crate::Error> {
    let mut result = Err(Error::Unknown);
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
