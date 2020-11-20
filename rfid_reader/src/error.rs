use crate::id::{ProductId, VendorId};
use rusb;
#[derive(Debug, PartialEq)]
pub enum Error {
    DeviceNotFound(VendorId, ProductId),
    Timeout,
    Access,
    ReadableInterruptEndPointNotFound(VendorId, ProductId),
    InvalidData,
    TooFewReceivedData,
    KeyNotExisting(u8),
    RusbError(rusb::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<rusb::Error> for Error {
    fn from(error: rusb::Error) -> Self {
        match error {
            rusb::Error::Timeout => Self::Timeout,
            rusb::Error::Access => Self::Access,
            _ => Self::RusbError(error),
        }
    }
}
