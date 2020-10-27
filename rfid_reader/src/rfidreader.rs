use crate::usbreader::{UsbReader,UsbReaderError};
use crate::keymap::KeyMap;
use crate::keymaperror::KeyMapError;

pub struct RfidReader {
  keymap: KeyMap,
  usbreader: UsbReader,
}

pub enum RfidReaderError {
  DeviceNotFound,
}

// impl RfidReader {
//   pub fn open(vendor_id:u16, product_id:u16, timeout:Duration) -> Result<RfidReader, RfidReaderError> {
//     let usb_reader = 
//   }
// }