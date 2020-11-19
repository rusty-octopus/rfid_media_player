use crate::error::Error;
use crate::id::{ProductId, VendorId};
use crate::libusbutils::{
    configure_device_handle, get_device, get_readable_interrupt_endpoint, EndPoint,
};
use crate::usbreader::UsbReader;
use libusb::Context;
use std::time::Duration;

pub(crate) struct NeuftechUsbReader {
    kernel_driver_attached: bool,
    endpoint: EndPoint,
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
}

impl std::fmt::Debug for NeuftechUsbReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeuftechUsbReader").finish()
    }
}

impl NeuftechUsbReader {
    pub(crate) fn open(
        vendor_id: VendorId,
        product_id: ProductId,
        timeout: Duration,
    ) -> Result<Self, Error> {
        let context = Context::new()?;
        let (device, device_descriptor) = get_device(&context, vendor_id, product_id)?;
        let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor)?;
        let mut device_handle = device.open()?;
        let kernel_driver_attached =
            device_handle.kernel_driver_active(endpoint.get_interface())?;
        if kernel_driver_attached {
            device_handle.detach_kernel_driver(endpoint.get_interface())?;
        }
        configure_device_handle(&mut device_handle, &endpoint)?;
        Ok(NeuftechUsbReader {
            kernel_driver_attached,
            endpoint,
            vendor_id,
            product_id,
            timeout,
        })
    }
}

impl Drop for NeuftechUsbReader {
    fn drop(&mut self) {
        if self.kernel_driver_attached {
            let context = Context::new().unwrap();
            let (device, _) = get_device(&context, self.vendor_id, self.product_id).unwrap();
            let mut device_handle = device.open().unwrap();
            device_handle
                .attach_kernel_driver(self.endpoint.get_interface())
                .unwrap();
            device_handle
                .release_interface(self.endpoint.get_interface())
                .unwrap();
        }
    }
}

/*
  libusbutils holds all convenience functions
  NeuftechUsbReader or rawusbreader holds Context
  on each new read, the device handle is opened again and made available again
  Endpoint etc. is also stored
  maybe on drop or on each read the kernel is attached again
  Advanced option: Use thread and send data via channel, then on spawning the thread context,
  device handle etc. can be kept alive in thread
*/

impl UsbReader for NeuftechUsbReader {
    fn read(&self) -> Result<Box<[u8]>, Error> {
        let context = Context::new()?;
        let (device, _) = get_device(&context, self.vendor_id, self.product_id)?;
        let device_handle = device.open()?;
        let mut raw_data_interpreter = RawDataInterpreter::default();
        let mut buffer = [0; 3];
        while !raw_data_interpreter.finished_processing() {
            let result = device_handle.read_interrupt(
                self.endpoint.get_address(),
                &mut buffer,
                self.timeout,
            );
            if result.is_ok() {
                raw_data_interpreter.process(&buffer)?;
            }
        }
        Ok(Box::new(raw_data_interpreter.data))
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
                _ => Err(Error::InvalidData),
            };
            return return_value;
        }
        Err(Error::TooFewReceivedData)
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
mod tests {
    use super::*;

    #[test]
    fn test_raw_data_interpretation() {
        let data: [u8; 1] = [0];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Err(Error::TooFewReceivedData), result);

        let data: [u8; 3] = [1, 0, 39];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Value(39)), result);

        let data: [u8; 3] = [1, 0, 40];
        let result = RawDataInterpretation::from(&data);
        assert_eq!(Ok(RawDataInterpretation::Enter), result);
    }

    #[test]
    fn test_raw_data_interpreter() {
        let mut interpreter = RawDataInterpreter::default();
        let test_data = [1, 0, 39];
        for _ in 0..=9 {
            assert_eq!(Ok(()), interpreter.process(&test_data));
            assert!(!interpreter.finished_processing());
        }
        let enter_data = [1, 0, 40];
        interpreter.process(&enter_data).unwrap();
        assert!(!interpreter.finished_processing());

        let ignore_data = [1, 0, 0];
        interpreter.process(&ignore_data).unwrap();
        assert!(interpreter.finished_processing());
    }
}
