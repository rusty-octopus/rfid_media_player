use log::{debug, error, info, warn};

use media_player::MediaPlayer;
use rfid_reader::RfidReader;
use track_store::TrackStore;

pub trait RfidMediaPlayer {
    fn run(&mut self);
    fn shutdown(&mut self);
}

struct RfidMediaPlayerImplementation<M, R, T>
where
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    media_player: M,
    rfid_reader: R,
    track_store: T,
}

pub(crate) fn open<M, R, T>(media_player: M, rfid_reader: R, track_store: T) -> impl RfidMediaPlayer
where
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    RfidMediaPlayerImplementation::open(media_player, rfid_reader, track_store)
}

impl<M, R, T> RfidMediaPlayer for RfidMediaPlayerImplementation<M, R, T>
where
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    fn run(&mut self) {
        let read_result = self.rfid_reader.read();
        match read_result {
            Ok(rfid_value) => {
                info!("Received RFID value: {}", rfid_value);
                let option_track_path = get_track(&self.track_store, rfid_value);
                if let Some(track_path) = option_track_path {
                    play_track(&mut self.media_player, track_path)
                }
            }
            Err(error) => match error {
                rfid_reader::Error::Timeout => {
                    debug!(
                        "Timeout error occurred ({}) which will be recovered.",
                        error
                    );
                    // enable callee to be non-blocking
                    return;
                }
                _ => {
                    error!("Reading RFID resolved in error: {}", error);
                }
            },
        }
    }

    fn shutdown(&mut self) {
        let rfid_reader_deinit_result = self.rfid_reader.deinitialize();
        if let Err(error) = rfid_reader_deinit_result {
            error!(
                "RFID reader could not be deinitialized without error: {}",
                error
            );
        }
        let media_player_stop_result = self.media_player.stop();
        if let Err(error) = media_player_stop_result {
            error!("Stopping media player resulted in error: {}", error);
        }
    }
}

impl<M, R, T> RfidMediaPlayerImplementation<M, R, T>
where
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    fn open(media_player: M, rfid_reader: R, track_store: T) -> Self {
        RfidMediaPlayerImplementation {
            media_player: media_player,
            rfid_reader: rfid_reader,
            track_store: track_store,
        }
    }
}

fn read_rfid(rfid_reader: &impl RfidReader) -> Result<String, rfid_reader::Error> {
    let read_result = rfid_reader.read();
    match read_result {
        Ok(rfid_value) => {
            info!("Received RFID value: {}", rfid_value);
            Ok(rfid_value)
        }
        Err(error) => match error {
            rfid_reader::Error::Timeout => {
                debug!(
                    "Timeout error occurred ({}) which will be recovered.",
                    error
                );
                Err(error)
            }
            _ => {
                error!("Reading RFID resolved in error: {}", error);
                Err(error)
            }
        },
    }
}

fn get_track<'a>(
    track_store: &'a impl TrackStore,
    rfid_value: String,
) -> Option<&'a track_store::TrackPath> {
    debug!("Get track for rfid {}", rfid_value);
    let id = track_store::Id::from(rfid_value);
    debug!("Converted to id {}", id);
    let option = track_store.get_path(&id);
    debug!("Optional path to id {:?}", option);
    match option {
        Some(track_path) => {
            info!("Found track {} for RFID {}.", track_path, id);
            Some(track_path)
        }
        None => {
            warn!("found no track for RFID {}.", id);
            None
        }
    }
}

fn play_track(media_player: &mut impl MediaPlayer, track_path: &track_store::TrackPath) {
    let track: media_player::Track = media_player::Track::from(track_path.as_ref());
    let play_result = media_player.play(&track);
    match play_result {
        Ok(()) => info!("Start playing track {}", track),
        Err(error) => error!(
            "Track {} could not be played, received error: {}",
            track, error
        ),
    };
}
