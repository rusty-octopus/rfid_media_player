use rodio::{OutputStream, Decoder, Sink};

use std::fs::File;
use std::io::BufReader;

use crate::audiolib::AudioLib;
use crate::error::Error;
use crate::track::Track;

struct RodioLib {
    sink: Sink
}

impl RodioLib {
  fn new() -> Result<Self,rodio::PlayError> {
    // Todo error
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle)?;
    Ok(RodioLib {
      sink: sink
    })
  }
}

impl AudioLib for RodioLib {
  fn play(&self, track: &Track) -> Result<(), Error> {
    // TODO: Implement with ? operator
    let file = File::open("sound.flac").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
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

pub(crate) fn open() -> impl AudioLib {
    let lib = RodioLib::new().unwrap();
    lib
}