use crate::error::Error;
pub trait MediaPlayer {
    fn play(&mut self, track: String) -> Result<(), Error>;
    fn close(&mut self) -> Result<(), Error>;
}

struct MediaPlayerImplementation {
    currently_playing: bool,
    last_song: Option<String>, // <-- Change to New Type
}

impl MediaPlayer for MediaPlayerImplementation {
    fn play(&mut self, track: String) -> Result<(), Error> {
        todo!();
    }
    fn close(&mut self) -> Result<(), Error> {
        todo!();
    }
}

pub fn open() -> Result<impl MediaPlayer, Error> {
    Ok(MediaPlayerImplementation {
        currently_playing: false,
        last_song: None,
    })
}
