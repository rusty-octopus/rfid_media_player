#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;

use log::{debug, error, info, warn};

use media_player::MediaPlayer;
use rfid_reader::RfidReader;
use track_store::TrackStore;

/// The `RfidMediaPlayer` trait.
pub trait RfidMediaPlayer {
    /// `run`s the `RfidMediaPlayer`.
    ///
    /// Returns either a success or an [`Error`](crate::Error).
    /// Implementors must implement run non-blocking in order to allow the callee
    /// to stop the application on terminated signals etc.
    fn run(&mut self) -> Result<(), Error>;
    /// `shutdown`s the `RfidMediaPlayer`.
    ///
    /// Returns either a success or an [`Error`](crate::Error).
    /// Must be called before the application is stopped.
    fn shutdown(&mut self) -> Result<(), Error>;
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

/// Opens the `RfidMediaPlayer`.
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
    fn run(&mut self) -> Result<(), Error> {
        let mut result = Ok(());
        let read_result = self.rfid_reader.read();
        match read_result {
            Ok(rfid_value) => {
                info!("Received RFID value: {}", rfid_value);
                let option_track_path = get_track(&self.track_store, rfid_value);
                if let Some(track_path) = option_track_path {
                    result = play_track(&mut self.media_player, track_path);
                }
            }
            Err(error) => match error {
                rfid_reader::Error::Timeout => result = Ok(()),
                _ => {
                    error!("Reading RFID resolved in error: {}", error);
                    result = Err(Error::from(error));
                }
            },
        }
        result
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        let mut result = Ok(());
        let rfid_reader_deinit_result = self.rfid_reader.deinitialize();
        if let Err(error) = rfid_reader_deinit_result {
            error!(
                "RFID reader could not be deinitialized without error: {}",
                error
            );
            result = Err(Error::from(error))
        }
        let media_player_stop_result = self.media_player.stop();
        if let Err(error) = media_player_stop_result {
            error!("Stopping media player resulted in error: {}", error);
            result = Err(Error::from(error))
        }
        result
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

fn play_track(
    media_player: &mut impl MediaPlayer,
    track_path: &track_store::TrackPath,
) -> Result<(), Error> {
    let track: media_player::Track = media_player::Track::from(track_path.as_ref());
    let play_result = media_player.play(&track);
    match play_result {
        Ok(()) => {
            info!("Start playing track {}", track);
            Ok(())
        }
        Err(error) => {
            error!(
                "Track {} could not be played, received error: {}",
                track, error
            );
            Err(Error::from(error))
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    struct OkMediaPlayer;
    impl MediaPlayer for OkMediaPlayer {
        fn play(&mut self, _track: &media_player::Track) -> Result<(), media_player::Error> {
            Ok(())
        }
        fn stop(&mut self) -> Result<(), media_player::Error> {
            Ok(())
        }
    }

    struct ErrMediaPlayer;
    impl MediaPlayer for ErrMediaPlayer {
        fn play(&mut self, _track: &media_player::Track) -> Result<(), media_player::Error> {
            Err(media_player::Error::AudioLibError("play".to_string()))
        }
        fn stop(&mut self) -> Result<(), media_player::Error> {
            Err(media_player::Error::AudioLibError("stop".to_string()))
        }
    }

    struct SomeTrackStore(track_store::TrackPath);
    impl TrackStore for SomeTrackStore {
        fn get_path(&self, _id: &track_store::Id) -> Option<&track_store::TrackPath> {
            Some(&self.0)
        }
    }

    struct NoneTrackStore;
    impl TrackStore for NoneTrackStore {
        fn get_path(&self, _id: &track_store::Id) -> Option<&track_store::TrackPath> {
            None
        }
    }

    #[derive(Debug)]
    struct OkRfidReader;
    impl RfidReader for OkRfidReader {
        fn read(&self) -> Result<String, rfid_reader::Error> {
            Ok("1234".to_string())
        }
        fn deinitialize(&mut self) -> Result<(), rfid_reader::Error> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct ErrRfidReader;
    impl RfidReader for ErrRfidReader {
        fn read(&self) -> Result<String, rfid_reader::Error> {
            Err(rfid_reader::Error::OtherUsbError("read".to_string()))
        }
        fn deinitialize(&mut self) -> Result<(), rfid_reader::Error> {
            Err(rfid_reader::Error::OtherUsbError(
                "deinitialize".to_string(),
            ))
        }
    }

    #[derive(Debug)]
    struct TimeoutRfidReader;
    impl RfidReader for TimeoutRfidReader {
        fn read(&self) -> Result<String, rfid_reader::Error> {
            Err(rfid_reader::Error::Timeout)
        }
        fn deinitialize(&mut self) -> Result<(), rfid_reader::Error> {
            Ok(())
        }
    }

    #[test]
    fn test_play_track() {
        let mut ok = OkMediaPlayer;
        let result = play_track(&mut ok, &track_store::TrackPath::from(""));
        assert_eq!(Ok(()), result);

        let mut err = ErrMediaPlayer;
        let result = play_track(&mut err, &track_store::TrackPath::from(""));
        assert_eq!(
            Err(Error::MediaPlayerError(
                "AudioLibError(\"play\")".to_string()
            )),
            result
        );
    }
    #[test]
    fn test_get_track() {
        let some = SomeTrackStore(track_store::TrackPath::from("path"));
        let option = get_track(&some, "".to_string());
        assert_eq!(Some(&track_store::TrackPath::from("path")), option);

        let none = NoneTrackStore;
        let option = get_track(&none, "".to_string());
        assert_eq!(None, option);
    }

    #[test]
    fn test_ok_run_and_shutdown() {
        let mut rfid_media_player = open(
            OkMediaPlayer,
            OkRfidReader,
            SomeTrackStore(track_store::TrackPath::from("path")),
        );

        let result = rfid_media_player.run();

        assert_eq!(Ok(()), result);

        let result = rfid_media_player.shutdown();

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_err_run_and_shutdown() {
        let mut rfid_media_player = open(
            OkMediaPlayer,
            ErrRfidReader,
            SomeTrackStore(track_store::TrackPath::from("path")),
        );

        let result = rfid_media_player.run();

        assert_eq!(
            Err(Error::RfidReaderError(
                "OtherUsbError(\"read\")".to_string()
            )),
            result
        );

        let result = rfid_media_player.shutdown();

        assert_eq!(
            Err(Error::RfidReaderError(
                "OtherUsbError(\"deinitialize\")".to_string()
            )),
            result
        );
    }

    #[test]
    fn test_err_media_player_shutdown() {
        let mut rfid_media_player = open(
            ErrMediaPlayer,
            OkRfidReader,
            SomeTrackStore(track_store::TrackPath::from("path")),
        );

        let result = rfid_media_player.shutdown();

        assert_eq!(
            Err(Error::MediaPlayerError(
                "AudioLibError(\"stop\")".to_string()
            )),
            result
        );
    }

    #[test]
    fn test_timeout_run() {
        let mut rfid_media_player = open(
            OkMediaPlayer,
            TimeoutRfidReader,
            SomeTrackStore(track_store::TrackPath::from("path")),
        );

        let result = rfid_media_player.run();

        assert_eq!(Ok(()), result);
    }
}
