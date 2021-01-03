#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

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
        !self.sink.empty()
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
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_create_triple() {
        let result = create_new_triple();
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_playing() {
        let mut rodio_lib = open().unwrap();
        let is_playing = rodio_lib.is_playing();
        assert_eq!(false, is_playing);

        let track = Track::from("tests/rand1.wav");
        rodio_lib.play(&track).unwrap();
        assert_eq!(true, rodio_lib.is_playing());

        rodio_lib.stop().unwrap();
        assert_eq!(false, rodio_lib.is_playing());
    }

    #[test]
    #[ignore = "Test of correct usage of rodio"]
    fn test_is_playing_to_the_end() {
        let mut rodio_lib = open().unwrap();
        let is_playing = rodio_lib.is_playing();
        assert_eq!(false, is_playing);

        let track = Track::from("tests/rand1.wav");
        rodio_lib.play(&track).unwrap();
        assert_eq!(true, rodio_lib.is_playing());

        for i in 0..32 {
            std::thread::sleep(Duration::from_secs(1));
            println!("Time elapsed: {} s", i + 1);
            assert_eq!(true, rodio_lib.is_playing());
        }
        std::thread::sleep(Duration::from_secs(3));
        assert_eq!(false, rodio_lib.is_playing());
    }

    #[test]
    fn test_from_play_error() {
        let rodio_error = rodio::PlayError::NoDevice;
        let error = Error::from(rodio_error);
        assert_eq!(Error::AudioLibError(String::from("NoDevice")), error);
    }

    #[test]
    fn test_from_stream_error() {
        let rodio_error = rodio::StreamError::NoDevice;
        let error = Error::from(rodio_error);
        assert_eq!(Error::AudioLibError(String::from("NoDevice")), error);
    }

    #[test]
    fn test_from_devices_error() {
        extern crate cpal;

        let rodio_error = rodio::DevicesError::BackendSpecific {
            err: cpal::BackendSpecificError {
                description: String::from(""),
            },
        };
        let error = Error::from(rodio_error);
        assert_eq!(
            Error::AudioLibError(String::from("A backend-specific error has occurred: ")),
            error
        );
    }

    #[test]
    fn test_from_decoder_error() {
        let rodio_error = rodio::decoder::DecoderError::UnrecognizedFormat;
        let error = Error::from(rodio_error);
        assert_eq!(
            Error::AudioLibError(String::from("Unrecognized format")),
            error
        );
    }
}
