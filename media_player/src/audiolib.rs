#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::track::Track;

pub(crate) trait AudioLib {
    fn play(&self, track: &Track) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn is_playing(&self) -> bool;
}
