#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::id::{ProductId, VendorId};
/// RFID Reader errors.
///
/// Defines all runtime errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Device Not Found.
    ///
    /// Returned whenever the device defined by [VendorId](crate::VendorId) and [ProductId](crate::ProductId).
    DeviceNotFound(VendorId, ProductId),
    /// Timeout error.
    ///
    /// The read returned a timeout. Mostly used internally.
    Timeout,
    /// Access.
    ///
    /// Access to Device denied. This usually happens when the user rights are not sufficient.
    /// Try using the RFID reader with more privileges.
    Access,
    /// Readable Endpoint Not Found.
    ///
    /// Returned whenever no readable endpoint could be found for the device defined by [VendorId](crate::VendorId) and [ProductId](crate::ProductId).
    ReadableEndPointNotFound(VendorId, ProductId),
    /// Invalid Data.
    ///
    /// Returned whenever invalid (or unexpected) data is received.
    InvalidData(u8),
    /// Too Few Received Data.
    ///
    /// Returned whenever too few bytes were received.
    TooFewReceivedData(usize),
    /// Key Not Existing.
    ///
    /// Returned whenever a byte value cannot be mapped to a key.
    KeyNotExisting(u8),
    /// Other Usb Error.
    ///
    /// Returned whenever an other error from the used USB library
    /// that is of minor importance for the RFID reader is returned.
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
