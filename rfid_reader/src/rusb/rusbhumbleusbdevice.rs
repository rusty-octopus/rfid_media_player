#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::humbleusbdevice::HumbleUsbDevice;
use crate::id::{ProductId, VendorId};
use crate::rusb::utils::{get_device, get_readable_endpoint, EndPoint};
use crate::Error;

use std::time::Duration;

use rusb::{Context, DeviceHandle, UsbContext};

struct RusbHumbleUsbDevice<T: UsbContext> {
    device_handle: DeviceHandle<T>,
    endpoint: EndPoint,
    attached_kernel_driver: bool,
    timeout: Duration,
}

impl<T: UsbContext> HumbleUsbDevice for RusbHumbleUsbDevice<T> {
    #[cfg(not(tarpaulin_include))]
    fn has_attached_kernel_driver(&self) -> bool {
        self.attached_kernel_driver
    }
    #[cfg(not(tarpaulin_include))]
    fn detach_kernel_driver(&mut self) -> Result<(), Error> {
        self.device_handle
            .detach_kernel_driver(self.endpoint.get_interface())?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn attach_kernel_driver(&mut self) -> Result<(), Error> {
        self.device_handle
            .attach_kernel_driver(self.endpoint.get_interface())?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn read(&self, buffer: &mut [u8]) -> Result<(), Error> {
        self.device_handle
            .read_interrupt(self.endpoint.get_address(), buffer, self.timeout)?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn claim_interface(&mut self) -> Result<(), Error> {
        self.device_handle
            .claim_interface(self.endpoint.get_interface())?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn release_interface(&mut self) -> Result<(), Error> {
        self.device_handle
            .release_interface(self.endpoint.get_interface())?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn set_active_configuration(&mut self) -> Result<(), Error> {
        self.device_handle
            .set_active_configuration(self.endpoint.get_config())?;
        Ok(())
    }
    #[cfg(not(tarpaulin_include))]
    fn set_alternate_setting(&mut self) -> Result<(), Error> {
        self.device_handle
            .set_alternate_setting(self.endpoint.get_interface(), self.endpoint.get_setting())?;
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
pub(crate) fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
) -> Result<impl HumbleUsbDevice, Error> {
    let context = Context::new()?;
    let (device, device_descriptor) = get_device(&context, vendor_id, product_id)?;
    let endpoint =
        get_readable_endpoint(&device, &device_descriptor, rusb::TransferType::Interrupt)?;
    let device_handle = device.open()?;
    let attached_kernel_driver = device_handle.kernel_driver_active(endpoint.get_interface())?;
    Ok(RusbHumbleUsbDevice {
        device_handle,
        endpoint,
        attached_kernel_driver,
        timeout,
    })
}
