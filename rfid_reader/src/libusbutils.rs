use crate::error::Error;
use crate::id::{ProductId, VendorId};
use libusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType};

#[derive(Debug, PartialEq)]
pub(crate) struct EndPoint {
    config: u8,
    interface: u8,
    setting: u8,
    address: u8,
}

impl EndPoint {
    pub(crate) fn get_interface(&self) -> u8 {
        self.config
    }
    pub(crate) fn get_address(&self) -> u8 {
        self.address
    }
}

pub(crate) fn configure_device_handle(
    device_handle: &mut DeviceHandle,
    end_point: &EndPoint,
) -> Result<(), Error> {
    device_handle.set_active_configuration(end_point.config)?;
    device_handle.claim_interface(end_point.interface)?;
    device_handle.set_alternate_setting(end_point.interface, end_point.setting)?;
    Ok(())
}

pub(crate) fn get_device(
    context: &Context,
    vendor_id: VendorId,
    product_id: ProductId,
) -> Result<(Device, DeviceDescriptor), Error> {
    let devices = context.devices()?;
    for device in devices.iter() {
        let device_descriptor = device.device_descriptor()?;
        if device_descriptor.vendor_id() == vendor_id.into()
            && device_descriptor.product_id() == product_id.into()
        {
            return Ok((device, device_descriptor));
        }
    }
    Err(Error::DeviceNotFound(vendor_id, product_id))
}

pub(crate) fn get_readable_interrupt_endpoint(
    device: &Device,
    device_descriptor: &DeviceDescriptor,
) -> Result<EndPoint, Error> {
    for n in 0..device_descriptor.num_configurations() {
        let config_description = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for interface in config_description.interfaces() {
            for interface_description in interface.descriptors() {
                for endpoint_descriptor in interface_description.endpoint_descriptors() {
                    if endpoint_descriptor.direction() == Direction::In
                        && endpoint_descriptor.transfer_type() == TransferType::Interrupt
                    {
                        return Ok(EndPoint {
                            config: config_description.number(),
                            interface: interface_description.interface_number(),
                            setting: interface_description.setting_number(),
                            address: endpoint_descriptor.address(),
                        });
                    }
                }
            }
        }
    }
    Err(Error::ReadableInterruptEndPointNotFound(
        device_descriptor.vendor_id().into(),
        device_descriptor.product_id().into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_device_not_found() {
        let context = Context::new().unwrap();
        let device = get_device(&context, VendorId::from(0), ProductId::from(0));
        assert!(device.is_err());
    }

    #[test]
    fn test_get_device() {
        let context = Context::new().unwrap();
        let device = get_device(&context, VendorId::from(0x0cf3), ProductId::from(0x3005));

        assert!(device.is_ok());
    }

    #[test]
    fn test_get_readable_endpoint() {
        let context = Context::new().unwrap();

        // readable device in my system, change these two values in your system
        let vendor_id = VendorId::from(0x0cf3);
        let product_id = ProductId::from(0x3005);
        let result = get_device(&context, vendor_id, product_id);
        assert!(result.is_ok());
        let (device, device_descriptor) = result.unwrap();
        let endpoint = get_readable_interrupt_endpoint(&device, &device_descriptor);
        assert!(endpoint.is_ok());
        assert_eq!(
            EndPoint {
                config: 1,
                interface: 0,
                setting: 0,
                address: 129
            },
            endpoint.unwrap()
        );
    }
}
