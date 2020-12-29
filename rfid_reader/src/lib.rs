//! A crate for a RFID Reader USB device.
//!
//! The RFID Reader can be opened to get a [RfidReader](crate::RfidReader).
//! One can read the processed data as String from this device.
//! It makes also sense to de-initialize the [RfidReader](crate::RfidReader) upon finishing.
//!
//! This crate is specifically implemented for a Neuftech RFID reader,
//! but may be extended to support other manufacturers.
//!
//! # Example
//! ```rust,no_run
//! use std::time::Duration;
//! use rfid_reader::{VendorId, ProductId, RfidReader};
//!
//! // set Vendor Id and Product Id
//! let vendor_id = VendorId::from(0x16c0);
//! let product_id = ProductId::from(0x27db);
//!
//! // Define a timeout (i.e. how long waits the reader for a RFID chip).
//! // From my experience, the longer the better, however it will block this amount of time.
//! let timeout = Duration::from_secs(60);
//! // get the rfid device, this may fail, for example when the device does not exist
//! let mut rfid_reader = rfid_reader::open(vendor_id, product_id, timeout).unwrap();
//!
//! // Read from the device. This is blocking!
//! // It should normally return a String containing numbers, but it may also fail.
//! // It will return Error::Timeout whenever the timeout has expired
//! let data = rfid_reader.read().unwrap();
//!
//! // Ideally de-initialize the device when no longer needed.
//! // This may fail therefore it makes sense to call it in order to get the error instead of
//! // a panic.
//! let result = rfid_reader.deinitialize();
//! if result.is_err() {
//!   println!("De-initialize failed: {}", result.unwrap_err());
//! }
//! ```
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use std::time::Duration;

mod error;
mod humbleusbdevice;
mod id;
mod keymap;
mod neuftech;
mod rfidreader;
mod rusb;
mod usbreader;

pub use error::Error;
pub use id::{ProductId, VendorId};
pub use rfidreader::RfidReader;

/// Tries to open an [RfidReader](crate::RfidReader).
///
/// Given the VendorId and the ProductId of the device, this function tries to open the device.
/// The `timeout` is used to wait blocking for an interrupt of the USB device.
/// From my experience: The longer the timeout the better.
///
/// May fail. Most important errors are [Error::DeviceNotFound](crate::error::Error::DeviceNotFound),
/// when the device could not be found. Or [Error::Access](crate::error::Error::Access) when there is
/// insufficient rights to open device.
#[cfg(not(tarpaulin_include))]
pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
) -> Result<impl RfidReader, Error> {
    let key_map = neuftech::NeuftechKeyMap;
    let rusb_device = rusb::open(vendor_id, product_id, timeout)?;
    let usb_reader = neuftech::new(rusb_device)?;
    Ok(rfidreader::from(key_map, usb_reader))
}
