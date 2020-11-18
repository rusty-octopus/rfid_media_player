use crate::error::Error;
use crate::id::{ProductId, VendorId};
use crate::keymap::{Key, KeyMap};
use crate::neuftech::{NeuftechKeyMap, NeuftechUsbReader};
use crate::usbreader::UsbReader;

use std::time::Duration;

pub trait RfidReader {
    fn read(&self) -> Result<String, Error>;
}

struct GenericRfidReader<K: KeyMap, U: UsbReader> {
    keymap: K,
    usbreader: U,
}

impl<K: KeyMap, U: UsbReader> GenericRfidReader<K, U> {
    fn from(keymap: K, usbreader: U) -> GenericRfidReader<K, U> {
        GenericRfidReader { keymap, usbreader }
    }
}

impl<K: KeyMap, U: UsbReader> RfidReader for GenericRfidReader<K, U> {
    fn read(&self) -> Result<String, Error> {
        let raw_data = self.usbreader.read()?;
        let mut rfid_value = String::with_capacity(10);
        for raw_value in raw_data.iter() {
            let key = self.keymap.map(*raw_value);
            if key.is_ok() {
                let key = key.unwrap();
                match key {
                    Key::Digit(c) => rfid_value.push(c),
                    _ => (),
                }
            }
        }
        Ok(rfid_value)
    }
}

pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
) -> Result<impl RfidReader, Error> {
    let keymap = NeuftechKeyMap;
    let usbreader = NeuftechUsbReader::open(vendor_id, product_id, timeout)?;
    Ok(GenericRfidReader::from(keymap, usbreader))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keymap::Key;

    struct MockUsbReader;

    impl UsbReader for MockUsbReader {
        fn read(&self) -> Result<Box<[u8]>, Error> {
            let data = (0..10).collect::<Vec<u8>>().into_boxed_slice();
            Ok(data)
        }
    }

    struct MockKeyMap;
    impl KeyMap for MockKeyMap {
        fn map(&self, key: u8) -> Result<Key, Error> {
            Ok(Key::Digit(
                std::char::from_digit(u8::into(key), 10).unwrap(),
            ))
        }
    }

    #[test]
    fn test_read() {
        let usb_reader = MockUsbReader;
        let key_map = MockKeyMap;
        let rfid_reader = GenericRfidReader::from(key_map, usb_reader);
        let rfid = rfid_reader.read().unwrap();
        assert_eq!("0123456789", rfid);
    }
}
