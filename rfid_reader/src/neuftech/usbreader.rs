#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::humbleusbdevice::HumbleUsbDevice;

use crate::usbreader::UsbReader;

pub(crate) struct NeuftechUsbReader<T>
where
    T: HumbleUsbDevice,
{
    usb_device: T,
}

impl<T: HumbleUsbDevice> std::fmt::Debug for NeuftechUsbReader<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeuftechUsbReader").finish()
    }
}

pub(crate) fn new<T: HumbleUsbDevice>(humble_usb_device: T) -> Result<impl UsbReader, Error> {
    NeuftechUsbReader::new(humble_usb_device)
}

impl<T: HumbleUsbDevice> NeuftechUsbReader<T> {
    fn new(usb_device: T) -> Result<Self, Error> {
        let mut usb_device = usb_device;
        usb_device.initialize()?;
        Ok(NeuftechUsbReader { usb_device })
    }
}

impl<T: HumbleUsbDevice> UsbReader for NeuftechUsbReader<T> {
    fn read(&self) -> Result<Box<[u8]>, Error> {
        let mut raw_data_interpreter = RawDataInterpreter::default();
        let mut buffer = [0; 3];
        while !raw_data_interpreter.finished_processing() {
            let result = self.usb_device.read(&mut buffer);
            if result.is_ok() {
                raw_data_interpreter.process(&buffer)?;
            } else {
                let error = result.unwrap_err();
                return Err(error);
            }
        }
        Ok(Box::new(raw_data_interpreter.data))
    }
    fn deinitialize(&mut self) -> Result<(), Error> {
        self.usb_device.deinitialize()
    }
}

#[derive(Debug, PartialEq)]
enum RawDataInterpretation {
    Value(u8),
    Repeated,
    Enter,
}

impl RawDataInterpretation {
    fn from(data: &[u8]) -> Result<RawDataInterpretation, Error> {
        if data.len() >= 3 {
            let value = data[2];
            let return_value = match value {
                0 => Ok(Self::Repeated),
                30..=39 => Ok(Self::Value(value)),
                40 => Ok(Self::Enter),
                _ => Err(Error::InvalidData(value)),
            };
            return return_value;
        }
        Err(Error::TooFewReceivedData(data.len()))
    }
}

struct RawDataInterpreter {
    finished: bool,
    index: usize,
    data: [u8; 10],
    last: Option<RawDataInterpretation>,
}

impl Default for RawDataInterpreter {
    fn default() -> Self {
        RawDataInterpreter {
            finished: false,
            index: 0,
            data: [0; 10],
            last: None,
        }
    }
}

impl RawDataInterpreter {
    fn process(&mut self, raw_data: &[u8]) -> Result<(), Error> {
        let raw_data_interpretation = RawDataInterpretation::from(raw_data)?;
        match raw_data_interpretation {
            RawDataInterpretation::Value(value) => {
                self.data[self.index] = value;
                self.last = Some(raw_data_interpretation);
                self.index += 1;
            }
            RawDataInterpretation::Enter => {
                self.last = Some(raw_data_interpretation);
            }
            RawDataInterpretation::Repeated => {
                if self.index == 10 && self.last == Some(RawDataInterpretation::Enter) {
                    self.finished = true;
                }
            }
        }
        Ok(())
    }
    fn finished_processing(&self) -> bool {
        self.finished
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    struct ReadErrorHumbleUsbDevice {
        deinitialized: bool,
    }

    impl HumbleUsbDevice for ReadErrorHumbleUsbDevice {
        fn has_attached_kernel_driver(&self) -> Result<bool, Error> {
            Ok(true)
        }
        fn detach_kernel_driver(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn attach_kernel_driver(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn read(&self, buffer: &mut [u8]) -> Result<(), Error> {
            Err(Error::InvalidData(0))
        }
        fn claim_interface(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn release_interface(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_active_configuration(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_alternate_setting(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_deinitialized(&mut self) {
            self.deinitialized = true;
        }
        fn deinitialized(&self) -> bool {
            self.deinitialized
        }
    }

    #[test]
    fn test_raw_data_interpretation() {
        let data: [u8; 1] = [0];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Err(Error::TooFewReceivedData(1)), result);

        let data: [u8; 3] = [1, 0, 39];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Value(39)), result);

        let data: [u8; 3] = [1, 0, 40];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Enter), result);

        let data: [u8; 3] = [1, 0, 124];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Err(Error::InvalidData(124)), result);
    }

    #[test]
    fn test_raw_data_interpreter() {
        let mut interpreter = RawDataInterpreter::default();
        let test_data = [1, 0, 39];
        for _ in 0..=9 {
            assert_eq!(Ok(()), interpreter.process(&test_data));
            assert!(!interpreter.finished_processing());
        }
        assert_eq!(10, interpreter.index);
        let enter_data = [1, 0, 40];
        interpreter.process(&enter_data).unwrap();
        assert_eq!(interpreter.last, Some(RawDataInterpretation::Enter));
        assert!(!interpreter.finished_processing());

        let ignore_data = [1, 0, 0];
        interpreter.process(&ignore_data).unwrap();
        assert!(interpreter.finished_processing());
    }

    #[test]
    fn test_usb_reader_read_error() {
        let mut dummy_device = ReadErrorHumbleUsbDevice {
            deinitialized: false,
        };
        let mut usb_reader = new(dummy_device).unwrap();
        let result = usb_reader.read();
        assert_eq!(Err(Error::InvalidData(0)), result);
        usb_reader.deinitialize().unwrap();
    }

    #[test]
    fn test_debug() {
        let mut dummy_device = ReadErrorHumbleUsbDevice {
            deinitialized: false,
        };
        dummy_device.initialize().unwrap();
        let mut usb_reader = NeuftechUsbReader::new(dummy_device).unwrap();
        assert_eq!("NeuftechUsbReader", format!("{:?}", usb_reader));
        usb_reader.deinitialize().unwrap();
    }

    struct DummyHumbleUsbDevice {
        index: std::cell::Cell<usize>,
        enter_happened: std::cell::Cell<bool>,
        deinitialized: bool,
    }

    impl HumbleUsbDevice for DummyHumbleUsbDevice {
        fn has_attached_kernel_driver(&self) -> Result<bool, Error> {
            Ok(true)
        }
        fn detach_kernel_driver(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn attach_kernel_driver(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn read(&self, buffer: &mut [u8]) -> Result<(), Error> {
            let index = self.index.get();

            if index < 10 {
                buffer[2] = 30;
                self.index.set(index + 1);
            } else {
                let enter_happened = self.enter_happened.get();
                if enter_happened {
                    buffer[2] = 0;
                } else {
                    buffer[2] = 40;
                    self.enter_happened.set(true);
                }
            }

            Ok(())
        }
        fn claim_interface(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn release_interface(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_active_configuration(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_alternate_setting(&mut self) -> Result<(), Error> {
            Ok(())
        }
        fn set_deinitialized(&mut self) {
            self.deinitialized = false;
        }
        fn deinitialized(&self) -> bool {
            self.deinitialized
        }
    }

    #[test]
    fn test_usb_reader_successful_read() {
        let dummy_device = DummyHumbleUsbDevice {
            index: 0.into(),
            enter_happened: false.into(),
            deinitialized: false,
        };
        let usb_reader = NeuftechUsbReader::new(dummy_device).unwrap();
        let result = usb_reader.read();

        let expected_data: Vec<u8> = vec![30; 10];
        assert_eq!(expected_data, result.unwrap().into_vec());
    }
}
