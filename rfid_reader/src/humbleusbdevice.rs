use rusb::{Context, Device, DeviceHandle, UsbContext};

use crate::error::Error;
use crate::rusbutils::{get_device, get_readable_interrupt_endpoint, EndPoint};

use crate::id::{ProductId, VendorId};

pub trait HumbleUsbDevice {}

struct HumbleUsbDeviceImplementation<T: UsbContext> {
    context: T,
    device_handle: DeviceHandle<T>,
    endpoint: EndPoint,
}

impl<T: UsbContext> HumbleUsbDevice for HumbleUsbDeviceImplementation<T> {}

pub fn open(vendor_id: VendorId, product_id: ProductId) -> Result<impl HumbleUsbDevice, Error> {
    let context = Context::new()?;
    let (device, device_descriptor) = get_device(&context, vendor_id, product_id)?;
    let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor)?;
    let device_handle = device.open()?;
    Ok(HumbleUsbDeviceImplementation {
        context,
        device_handle,
        endpoint,
    })
}
