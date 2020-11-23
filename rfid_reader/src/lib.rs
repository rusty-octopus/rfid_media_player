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

#[cfg(not(tarpaulin_include))]
pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
) -> Result<impl RfidReader, Error> {
    let key_map = neuftech::NeuftechKeyMap;
    let rusb_device = rusb::open(vendor_id, product_id, timeout)?;
    let usb_reader = neuftech::new(rusb_device)?;
    Ok(rfidreader::new(key_map, usb_reader))
}
