#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::audiolib::AudioLib;
use crate::error::Error;
use crate::track::Track;
/// The [`MediaPlayer`](crate::MediaPlayer) trait.
///
/// Enables playing tracks and stopping the playback.
/// When a new track is played, the playback of the old one is stopped (if still playing).
pub trait MediaPlayer {
    /// Plays the [`Track`].
    ///
    /// Results in an [`IoError`](crate::Error::IoError) if the file does not exist (or any other possible file access error).
    /// Results in an [`AudioLibError`](crate::Error::AudioLibError) if there was an error raised by the underlying audio library.
    fn play(&mut self, track: &Track) -> Result<(), Error>;

    /// Stops the playback.
    ///
    /// Results in an [`AudioLibError`](crate::Error::AudioLibError) if there was an error raised by the underlying audio library.
    fn stop(&mut self) -> Result<(), Error>;
}

struct MediaPlayerImplementation<T: AudioLib> {
    last_track: Option<Track>,
    audio_lib: T,
}

impl<T: AudioLib> MediaPlayer for MediaPlayerImplementation<T> {
    fn play(&mut self, track: &Track) -> Result<(), Error> {
        if let Some(last_track) = &self.last_track {
            if last_track != track {
                self.audio_lib.stop()?;
                self.audio_lib.play(&track)?;
                self.last_track = Some(track.clone());
            } else {
                if !self.audio_lib.is_playing() {
                    self.audio_lib.play(&track)?;
                }
            }
        } else {
            self.audio_lib.play(&track)?;
            self.last_track = Some(track.clone());
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.last_track = None;
        self.audio_lib.stop()
    }
}

impl<T: AudioLib> MediaPlayerImplementation<T> {
    fn from(audio_lib: T) -> Result<Self, Error> {
        Ok(MediaPlayerImplementation {
            last_track: None,
            audio_lib: audio_lib,
        })
    }
}

pub(crate) fn open<T: AudioLib>(audio_lib: T) -> Result<impl MediaPlayer, Error> {
    MediaPlayerImplementation::from(audio_lib)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    struct DummyAudioLib;
    impl AudioLib for DummyAudioLib {
        fn play(&self, _track: &Track) -> Result<(), Error> {
            Ok(())
        }
        fn stop(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn is_playing(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_play() {
        let mut media_player = MediaPlayerImplementation::from(DummyAudioLib).unwrap();
        let result = media_player.play(&"/path/to/track".into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_song_interrupts_old() {
        let mut media_player = MediaPlayerImplementation::from(DummyAudioLib).unwrap();
        media_player.play(&"/path/to/track/1".into()).unwrap();
        media_player.play(&"song2".into()).unwrap();
        assert_eq!(media_player.last_track, Some(Track::from("song2")));
    }

    #[test]
    fn test_old_song_is_played_again() {
        let mut media_player = MediaPlayerImplementation::from(DummyAudioLib).unwrap();
        media_player
            .play(&String::from("/path/to/track/1").into())
            .unwrap();
        media_player.play(&"/path/to/track/1".into()).unwrap();
        assert_eq!(
            media_player.last_track,
            Some(Track::from("/path/to/track/1"))
        );
    }
}
