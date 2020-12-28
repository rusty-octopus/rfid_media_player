use media_player;
use rfid_reader::{ProductId, VendorId};
use std::time::Duration;
use track_store;

use log::error;

mod rfidmediaplayer;

pub use rfidmediaplayer::RfidMediaPlayer;

pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
    yaml_string: &str,
) -> Option<impl RfidMediaPlayer> {
    let mut option_rfid_reader = None;
    let result_track_store = track_store::load(yaml_string);
    if let Ok(track_store) = result_track_store {
        let result_rfid_reader = rfid_reader::open(vendor_id, product_id, timeout);
        if let Ok(rfid_reader) = result_rfid_reader {
            let result_media_player = media_player::open();
            if let Ok(media_player) = result_media_player {
                option_rfid_reader = Some(rfidmediaplayer::open(
                    media_player,
                    rfid_reader,
                    track_store,
                ));
            } else if let Err(error) = result_media_player {
                error!("Opening media player resulted in error: {}", error);
            }
        } else if let Err(error) = result_rfid_reader {
            error!("Opening rfid reader resulted in error: {}", error);
        }
    } else if let Err(error) = result_track_store {
        error!("Loading track store resulted in error: {}", error);
    }

    option_rfid_reader
}
