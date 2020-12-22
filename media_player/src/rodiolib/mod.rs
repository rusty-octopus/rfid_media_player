use rodio::{OutputStream, OutputStreamHandle, Sink};

use std::fs::File;
use std::io::BufReader;

use crate::audiolib::AudioLib;
use crate::error::Error;
use crate::track::Track;

struct RodioLib {
    sink: Sink,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

fn create_new_triple() -> Result<(Sink, OutputStream, OutputStreamHandle), Error> {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle)?;
    Ok((sink, stream, stream_handle))
}

impl RodioLib {
    fn new() -> Result<Self, Error> {
        let (sink, stream, stream_handle) = create_new_triple()?;

        Ok(RodioLib {
            sink: sink,
            stream: stream,
            stream_handle: stream_handle,
        })
    }
}

impl AudioLib for RodioLib {
    fn play(&self, track: &Track) -> Result<(), Error> {
        let file = File::open(track)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        self.sink.play();
        Ok(())
    }
    fn stop(&mut self) -> Result<(), Error> {
        self.sink.stop();
        let (sink, stream, stream_handle) = create_new_triple()?;
        self.sink = sink;
        self.stream = stream;
        self.stream_handle = stream_handle;
        Ok(())
    }
    fn is_playing(&self) -> bool {
        self.sink.empty()
    }
}

pub(crate) fn open() -> Result<impl AudioLib, Error> {
    RodioLib::new()
}

impl From<rodio::PlayError> for Error {
    fn from(error: rodio::PlayError) -> Self {
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

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_create_triple() {
        let result = create_new_triple();
        assert!(result.is_ok());
    }
}
