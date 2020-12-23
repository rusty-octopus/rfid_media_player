//! Crate for playing audio files.
//!
//! The [`MediaPlayer`](crate::MediaPlayer) can simply be
//! [`open`](crate::open)ed but may result in an error.
//!
//! # Example
//!
//! ```rust
//! use media_player::{open, MediaPlayer, Track};
//!
//! // get media_player trait object
//! let mut media_player = open().unwrap();
//!
//! // create Track from String or &str
//! let track = Track::from("tests/rand1.wav");
//!
//! // play the track
//! media_player.play(&track).unwrap();
//!
//! // ...
//!
//! // if you play a new track, the old one is stopped
//! let track2 = Track::from("tests/rand2.wav");
//! media_player.play(&track2).unwrap();
//!
//! // ...
//!
//! // you can also stop the playing of the track
//! media_player.stop().unwrap();
//! ```
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

mod audiolib;
mod error;
mod mediaplayer;
mod track;

mod rodiolib;

pub use error::Error;
pub use mediaplayer::MediaPlayer;
pub use track::Track;

/// Opens the [`MediaPlayer`](crate::MediaPlayer), can result in an Error.
///
/// Returns an [`AudioLibError`](crate::Error::AudioLibError) if there was
/// an error raised in the underlying audio library.
pub fn open() -> Result<impl MediaPlayer, Error> {
    let audiolib = rodiolib::open()?;
    mediaplayer::open(audiolib)
}
