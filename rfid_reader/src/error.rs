#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::id::{ProductId, VendorId};
use rusb;
#[derive(Debug, PartialEq)]
pub enum Error {
    DeviceNotFound(VendorId, ProductId),
    Timeout,
    Access,
    ReadableEndPointNotFound(VendorId, ProductId),
    InvalidData,
    TooFewReceivedData,
    KeyNotExisting(u8),
    OtherUsbError(String),
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
            _ => Self::OtherUsbError(error.to_string()),
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = Error::Timeout;
        assert_eq!("Timeout", format!("{}", error));
    }

    #[test]
    fn test_from_rusb_error() {
        let rusb_error = rusb::Error::Io;
        let error = Error::from(rusb_error);
        assert_eq!(
            Error::OtherUsbError(String::from("Input/Output Error")),
            error
        );
        let rusb_error = rusb::Error::Timeout;
        let error = Error::from(rusb_error);
        assert_eq!(Error::Timeout, error);
        let rusb_error = rusb::Error::Access;
        let error = Error::from(rusb_error);
        assert_eq!(Error::Access, error);
    }
}
