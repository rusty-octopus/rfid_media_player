#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::id::{ProductId, VendorId};
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

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = Error::Timeout;
        assert_eq!("Timeout", format!("{}", error));
    }
}
