use crate::audiolib::AudioLib;
use crate::error::Error;
use crate::track::Track;
pub trait MediaPlayer {
    fn play(&mut self, track: &Track) -> Result<(), Error>;
}

struct MediaPlayerImplementation<T: AudioLib> {
    last_track: Option<Track>,
    audiolib: T,
}

impl<T: AudioLib> MediaPlayer for MediaPlayerImplementation<T> {
    fn play(&mut self, track: &Track) -> Result<(), Error> {
        if let Some(last_track) = &self.last_track {
            if last_track != track {
                self.audiolib.play(&track)?;
                self.last_track = Some(track.clone());
            } else {
                if !self.audiolib.is_playing() {
                    self.audiolib.play(&track)?;
                }
            }
        } else {
            self.audiolib.play(&track)?;
            self.last_track = Some(track.clone());
        }
        Ok(())
    }
}

impl<T: AudioLib> MediaPlayerImplementation<T> {
    fn from(audiolib: T) -> Result<Self, Error> {
        Ok(MediaPlayerImplementation {
            last_track: None,
            audiolib: audiolib,
        })
    }
}

struct DummyAudioLib;
impl AudioLib for DummyAudioLib {
    fn play(&self, track: &Track) -> Result<(), Error> {
        Ok(())
    }
    fn stop(&self) {}
    fn is_playing(&self) -> bool {
        false
    }
}

pub(crate) fn open<T: AudioLib>(audiolib: T) -> Result<impl MediaPlayer, Error> {
    MediaPlayerImplementation::from(audiolib)
}

pub(crate) fn open_dummy() -> Result<impl MediaPlayer, Error> {
    MediaPlayerImplementation::from(DummyAudioLib)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

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
        media_player.play(&String::from("/path/to/track/1").into());
        media_player.play(&"/path/to/track/1".into());
        assert_eq!(
            media_player.last_track,
            Some(Track::from("/path/to/track/1"))
        );
    }
}
