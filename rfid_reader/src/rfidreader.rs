use crate::keymap::{Key, KeyMap};
use crate::neuftech::{NeuftechKeyMap, NeuftechUsbReader};
use crate::usbreader::UsbReader;
use std::time::Duration;

pub trait RfidReader {
    fn read(&self) -> String;
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
    fn read(&self) -> String {
        let raw_data = self.usbreader.read();
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
        rfid_value
    }
}

pub enum RfidReaderError {
    DeviceNotFound,
}

pub fn open(
    vendor_id: u16,
    product_id: u16,
    timeout: Duration,
) -> Result<impl RfidReader, RfidReaderError> {
    let keymap = NeuftechKeyMap;
    let usbreader = NeuftechUsbReader;
    Ok(GenericRfidReader::from(keymap, usbreader))
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

    struct MockKeyMap;
    impl KeyMap for MockKeyMap {
        fn map(&self, key: u8) -> Result<Key, KeyMapError> {
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
        let rfid = rfid_reader.read();
        assert_eq!("0123456789", rfid);
    }
}
