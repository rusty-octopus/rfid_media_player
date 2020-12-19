mod audiolib;
mod error;
mod mediaplayer;
mod track;

mod rodiolib;

pub use error::Error;
pub use mediaplayer::MediaPlayer;
pub use track::Track;

pub fn open() -> Result<impl MediaPlayer, Error> {
    mediaplayer::open_dummy()
}
