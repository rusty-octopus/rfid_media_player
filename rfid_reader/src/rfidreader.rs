#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::keymap::{Key, KeyMap};
use crate::usbreader::UsbReader;

/// Trait defining an [RfidReader](crate::RfidReader).
///
/// An [RfidReader](crate::RfidReader) will read from an USB Rfid Reader device and convert the raw data into a string but may fail reading.
///
/// It also makes sense to call [`deinitialize`](crate::RfidReader::deinitialize) when the RfidReader is not used anymore in order to de-initialize the
/// used USB device. This makes sense since de-initialize may fail. However, Drop is implemented to de-initialize as well.
pub trait RfidReader: std::fmt::Debug {
    /// Tries to read from the RfidReader.
    ///
    /// A String of the processed raw data is returned on success. Otherwise an error is returned.
    /// Method is blocking, this means it will try to read data, until a valid RFID was read or an error has occurred.
    ///
    /// The error [`Timeout`](crate::Error::Timeout) will occur after the defined timeout expired.
    /// One can simply call this method again, since it is not a fatal error. However as mentioned above, the call to this method
    /// is blocking. This means when the [`Timeout`](crate::Error::Timeout) occurs, one can do something different like checking
    /// for OS signals that may signal that the application has to be terminated.
    fn read(&self) -> Result<String, Error>;
    /// Tries to de-initialize the USB device of the RfidReader, which may fail.
    fn deinitialize(&mut self) -> Result<(), Error>;
}

struct GenericRfidReader<K: KeyMap, U: UsbReader> {
    key_map: K,
    usb_reader: U,
}

impl<K: KeyMap, U: UsbReader> GenericRfidReader<K, U> {
    fn from(key_map: K, usb_reader: U) -> GenericRfidReader<K, U> {
        GenericRfidReader {
            key_map: key_map,
            usb_reader: usb_reader,
        }
    }
}

impl<K: KeyMap, U: UsbReader> std::fmt::Debug for GenericRfidReader<K, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericRfidReader")
            .field("keymap", &self.key_map)
            .field("usbreader", &self.usb_reader)
            .finish()
    }
}

impl<K: KeyMap, U: UsbReader> RfidReader for GenericRfidReader<K, U> {
    fn read(&self) -> Result<String, Error> {
        let raw_data = self.usb_reader.read()?;
        let mut rfid_value = String::with_capacity(10);
        for raw_value in raw_data.iter() {
            let key = self.key_map.map(*raw_value);
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
        self.usb_reader.deinitialize()
    }
}

pub(crate) fn from<K: KeyMap, U: UsbReader>(key_map: K, usb_reader: U) -> impl RfidReader {
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
        let mut rfid_reader = from(key_map, usb_reader);
        let rfid = rfid_reader.read().unwrap();
        assert_eq!("0123456789", rfid);
        assert_eq!(Ok(()), rfid_reader.deinitialize());
    }

    #[test]
    fn test_debug() {
        let usb_reader = MockUsbReader;
        let key_map = MockKeyMap;
        let rfid_reader = GenericRfidReader::from(key_map, usb_reader);
        assert_eq!("MockKeyMap", format!("{:?}", rfid_reader.key_map));
        assert_eq!(
            "GenericRfidReader { keymap: MockKeyMap, usbreader: MockUsbReader }",
            format!("{:?}", rfid_reader)
        );
    }
}
