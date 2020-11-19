use crate::error::Error;
pub(crate) trait UsbReader: std::fmt::Debug {
    fn read(&self) -> Result<Box<[u8]>, Error>;
}
