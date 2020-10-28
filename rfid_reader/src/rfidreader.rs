use crate::usbreader::UsbReader;
use crate::keymap::{KeyMap};
use std::time::Duration;

pub trait RfidReader {
  fn read(&self) -> String;
}

struct NeuftechRfidReader {
  keymap: Box<dyn KeyMap>,
  usbreader: Box<dyn UsbReader>,
}

pub enum RfidReaderError {
  DeviceNotFound,
}

pub fn open(vendor_id:u16, product_id:u16, timeout:Duration) -> Result<impl RfidReader, RfidReaderError> {
  NeuftechRfidReader::open(vendor_id, product_id, timeout)
}

impl NeuftechRfidReader {
  fn from(keymap: Box<dyn KeyMap>, usbreader : Box<dyn UsbReader>) -> impl RfidReader {
    NeuftechRfidReader{keymap, usbreader}
  }
  fn open(vendor_id:u16, product_id:u16, timeout:Duration) -> Result<NeuftechRfidReader, RfidReaderError> {
    todo!();
  }
}

impl RfidReader for NeuftechRfidReader {
  fn read(&self) -> String {
    todo!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::keymap::Key;
  use crate::keymaperror::KeyMapError;

  struct MockUsbReader;

  impl UsbReader for MockUsbReader {
    fn read(&self) -> Box<[u8]> {
      let data = (0..10).collect::<Vec<u8>>().into_boxed_slice();
      data
    }
  }


  struct MockKeyMap ;
  impl KeyMap for MockKeyMap {
    fn map(&self, key:u8) -> Result<Key,KeyMapError> {
      Ok(Key::Digit(std::char::from_digit(u8::into(key), 10).unwrap()))
    }
  }

  #[test]
  fn test_read() {
      let usb_reader = Box::from(MockUsbReader);
      let key_map = Box::from(MockKeyMap);
      let rfid_reader = NeuftechRfidReader::from(key_map, usb_reader);
      let rfid = rfid_reader.read();
  }
}