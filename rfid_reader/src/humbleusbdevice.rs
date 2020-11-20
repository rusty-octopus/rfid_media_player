use crate::error::Error;
use crate::id::{ProductId, VendorId};
use crate::rusbutils::{get_device, get_readable_interrupt_endpoint, EndPoint};

use std::time::Duration;

use rusb::{Context, DeviceHandle, UsbContext};

pub trait HumbleUsbDevice {
    fn has_attached_kernel_driver(&self) -> bool;
    fn detach_kernel_driver(&mut self) -> Result<(), Error>;
    fn attach_kernel_driver(&mut self) -> Result<(), Error>;
    fn read(&self, buffer: &mut [u8]) -> Result<(), Error>;
    fn claim_interface(&mut self) -> Result<(), Error>;
    fn release_interface(&mut self) -> Result<(), Error>;
    fn set_active_configuration(&mut self) -> Result<(), Error>;
    fn set_alternate_setting(&mut self) -> Result<(), Error>;
    fn configure_device(&mut self) -> Result<(), Error> {
        self.set_active_configuration()?;
        self.claim_interface()?;
        self.set_alternate_setting()?;
        Ok(())
    }
}

struct HumbleUsbDeviceImplementation<T: UsbContext> {
    device_handle: DeviceHandle<T>,
    endpoint: EndPoint,
    attached_kernel_driver: bool,
    timeout: Duration,
}

impl<T: UsbContext> HumbleUsbDevice for HumbleUsbDeviceImplementation<T> {
    fn has_attached_kernel_driver(&self) -> bool {
        self.attached_kernel_driver
    }
    fn detach_kernel_driver(&mut self) -> Result<(), Error> {
        self.device_handle
            .detach_kernel_driver(self.endpoint.get_interface())?;
        Ok(())
    }
    fn attach_kernel_driver(&mut self) -> Result<(), Error> {
        self.device_handle
            .attach_kernel_driver(self.endpoint.get_interface())?;
        Ok(())
    }
    fn read(&self, buffer: &mut [u8]) -> Result<(), Error> {
        self.device_handle
            .read_interrupt(self.endpoint.get_address(), buffer, self.timeout)?;
        Ok(())
    }
    fn claim_interface(&mut self) -> Result<(), Error> {
        self.device_handle
            .claim_interface(self.endpoint.get_interface())?;
        Ok(())
    }
    fn release_interface(&mut self) -> Result<(), Error> {
        self.device_handle
            .release_interface(self.endpoint.get_interface())?;
        Ok(())
    }
    fn set_active_configuration(&mut self) -> Result<(), Error> {
        self.device_handle
            .set_active_configuration(self.endpoint.get_config())?;
        Ok(())
    }
    fn set_alternate_setting(&mut self) -> Result<(), Error> {
        self.device_handle
            .set_alternate_setting(self.endpoint.get_interface(), self.endpoint.get_setting())?;
        Ok(())
    }
}

pub fn open(
    vendor_id: VendorId,
    product_id: ProductId,
    timeout: Duration,
) -> Result<impl HumbleUsbDevice, Error> {
    let context = Context::new()?;
    let (device, device_descriptor) = get_device(&context, vendor_id, product_id)?;
    let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor)?;
    let device_handle = device.open()?;
    let attached_kernel_driver = device_handle.kernel_driver_active(endpoint.get_interface())?;
    Ok(HumbleUsbDeviceImplementation {
        device_handle,
        endpoint,
        attached_kernel_driver,
        timeout,
    })
}
