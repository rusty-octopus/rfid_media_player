use crate::error::Error;
pub(crate) trait UsbReader {
    fn read(&self) -> Result<Box<[u8]>, Error>;
}
