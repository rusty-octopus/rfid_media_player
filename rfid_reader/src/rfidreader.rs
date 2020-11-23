#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::keymap::{Key, KeyMap};
use crate::usbreader::UsbReader;

pub trait RfidReader: std::fmt::Debug {
    fn read(&self) -> Result<String, Error>;
    fn deinitialize(&mut self) -> Result<(), Error>;
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

impl<K: KeyMap, U: UsbReader> std::fmt::Debug for GenericRfidReader<K, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericRfidReader")
            .field("keymap", &self.keymap)
            .field("usbreader", &self.usbreader)
            .finish()
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
                if let Key::Digit(c) = key {
                    rfid_value.push(c)
                }
            }
        }
        Ok(rfid_value)
    }
    fn deinitialize(&mut self) -> Result<(), Error> {
        self.usbreader.deinitialize()
    }
}

pub(crate) fn new<K: KeyMap, U: UsbReader>(key_map: K, usb_reader: U) -> impl RfidReader {
    GenericRfidReader::from(key_map, usb_reader)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::keymap::Key;

    struct MockUsbReader;

    impl UsbReader for MockUsbReader {
        fn read(&self) -> Result<Box<[u8]>, Error> {
            let data = (0..10).collect::<Vec<u8>>().into_boxed_slice();
            Ok(data)
        }
        fn deinitialize(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    impl std::fmt::Debug for MockUsbReader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockUsbReader").finish()
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

    impl std::fmt::Debug for MockKeyMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockKeyMap").finish()
        }
    }

    #[test]
    fn test_read() {
        let usb_reader = MockUsbReader;
        let key_map = MockKeyMap;
        let mut rfid_reader = new(key_map, usb_reader);
        let rfid = rfid_reader.read().unwrap();
        assert_eq!("0123456789", rfid);
        assert_eq!(Ok(()), rfid_reader.deinitialize());
    }

    #[test]
    fn test_debug() {
        let usb_reader = MockUsbReader;
        let key_map = MockKeyMap;
        let rfid_reader = GenericRfidReader::from(key_map, usb_reader);
        assert_eq!("MockKeyMap", format!("{:?}", rfid_reader.keymap));
        assert_eq!(
            "GenericRfidReader { keymap: MockKeyMap, usbreader: MockUsbReader }",
            format!("{:?}", rfid_reader)
        );
    }
}
