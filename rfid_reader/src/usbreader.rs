#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
pub(crate) trait UsbReader: std::fmt::Debug {
    fn read(&self) -> Result<Box<[u8]>, Error>;
    fn deinitialize(&mut self) -> Result<(), Error>;
}
