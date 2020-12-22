mod audiolib;
mod error;
mod mediaplayer;
mod track;

mod rodiolib;

pub use error::Error;
pub use mediaplayer::MediaPlayer;
pub use track::Track;

pub fn open() -> Result<impl MediaPlayer, Error> {
    let audiolib = rodiolib::open()?;
    mediaplayer::open(audiolib)
}
