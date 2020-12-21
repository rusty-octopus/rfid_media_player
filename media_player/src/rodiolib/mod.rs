use rodio::{OutputStream, PlayError, Sink};

use std::fs::File;
use std::io::BufReader;

use crate::audiolib::AudioLib;
use crate::error::Error;
use crate::track::Track;

struct RodioLib {
    sink: Sink,
}

impl RodioLib {
    fn new() -> Result<Self, Error> {
        // Todo error
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle)?;
        Ok(RodioLib { sink: sink })
    }
}

impl AudioLib for RodioLib {
    fn play(&self, track: &Track) -> Result<(), Error> {
        // TODO: Implement with ? operator
        let file = File::open("sound.flac")?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        self.sink.play();
        Ok(())
    }
    fn stop(&self) {
        self.sink.stop();
    }
    fn is_playing(&self) -> bool {
        self.sink.empty()
    }
}

pub(crate) fn open() -> Result<impl AudioLib, Error> {
    RodioLib::new()
}

impl From<rodio::PlayError> for Error {
    fn from(error: PlayError) -> Self {
        Error::AudioLibError(format!("{}", error))
    }
}

impl From<rodio::StreamError> for Error {
    fn from(error: rodio::StreamError) -> Self {
        Error::AudioLibError(format!("{}", error))
    }
}

impl From<rodio::DevicesError> for Error {
    fn from(error: rodio::DevicesError) -> Self {
        Error::AudioLibError(format!("{}", error))
    }
}

impl From<rodio::decoder::DecoderError> for Error {
    fn from(error: rodio::decoder::DecoderError) -> Self {
        Error::AudioLibError(format!("{}", error))
    }
}
