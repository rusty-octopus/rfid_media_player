use crate::id::{ProductId, VendorId};
use crate::libusbutils::{
    configure_device_handle, get_device, get_readable_interrupt_endpoint, EndPoint,
};
use crate::Error;
use libusb::{Context, Device, DeviceDescriptor, DeviceHandle};
use std::time::Duration;

pub(crate) trait RawUsbReader {
    fn open(context: &Context, vendor_id: VendorId, product_id: ProductId) -> Result<Self, Error>
    where
        Self: Sized;
    fn read(
        &self,
        device_handle: &DeviceHandle,
        buffer: &mut [u8],
        timeout: Duration,
    ) -> Result<usize, Error>;
    fn close(&self, device_handle: &DeviceHandle) -> Result<(), Error>;
}

pub(crate) struct RawInterruptUsbReader {
    kernel_driver_detached: bool,
    device_descriptor: DeviceDescriptor,
    endpoint: EndPoint,
}

pub(crate) struct TestReader<'a> {
    context: Context,
    device_handle: Option<DeviceHandle<'a>>,
}

//impl<'a> TestReader<'a> {
//    fn new() -> Result<Self, Error> {
//        Ok(TestReader {
//            context: Context::new()?,
//            device_handle: None,
//        })
//    }
//    fn open(&mut self, vendor_id: u16, product_id: u16) -> Result<(), Error> {
//        let (device, device_descriptor) = get_device(&self.context, vendor_id, product_id)?;
//        let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor)?;
//        let mut device_handle = device.open()?;
//        let kernel_driver_active = device_handle.kernel_driver_active(endpoint.get_interface())?;
//        if kernel_driver_active {
//            device_handle.detach_kernel_driver(endpoint.get_interface())?
//        }
//        configure_device_handle(&mut device_handle, &endpoint)?;
//        self.device_handle = Some(device_handle);
//        Ok(())
//    }
//}

impl RawUsbReader for RawInterruptUsbReader {
    fn open(context: &Context, vendor_id: VendorId, product_id: ProductId) -> Result<Self, Error> {
        let context = Context::new()?;
        let (device, device_descriptor) = get_device(&context, vendor_id, product_id)?;
        let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor)?;
        let mut device_handle = device.open()?;
        let kernel_driver_active = device_handle.kernel_driver_active(endpoint.get_interface())?;
        if kernel_driver_active {
            device_handle.detach_kernel_driver(endpoint.get_interface())?
        }
        configure_device_handle(&mut device_handle, &endpoint);
        Ok(RawInterruptUsbReader {
            kernel_driver_detached: kernel_driver_active,
            device_descriptor,
            endpoint,
        })
    }

    fn read(
        &self,
        device_handle: &DeviceHandle,
        buffer: &mut [u8],
        timeout: Duration,
    ) -> Result<usize, Error> {
        let read_bytes =
            device_handle.read_interrupt(self.endpoint.get_address(), buffer, timeout)?;
        Ok(read_bytes)
    }

    fn close(&self, device_handle: &DeviceHandle) -> Result<(), Error> {
        todo!();
    }
}
