use crate::error::Error;
use libusb::{Context, Device, DeviceDescriptor, Direction, TransferType};

#[derive(Debug, PartialEq)]
pub(crate) struct EndPoint {
    config: u8,
    interface: u8,
    setting: u8,
    address: u8,
}

fn get_device(
    context: &Context,
    vendor_id: u16,
    product_id: u16,
) -> Result<(Device, DeviceDescriptor), Error> {
    let devices = context.devices()?;
    for device in devices.iter() {
        let device_descriptor = device.device_descriptor()?;
        if device_descriptor.vendor_id() == vendor_id
            && device_descriptor.product_id() == product_id
        {
            return Ok((device, device_descriptor));
        }
    }
    Err(Error::DeviceNotFound(vendor_id, product_id))
}

fn get_readable_endpoint(
    context: &Context,
    device: &Device,
    device_descriptor: &DeviceDescriptor,
    transfer_type: TransferType,
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
                        && endpoint_descriptor.transfer_type() == transfer_type
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
    Err(Error::EndPointNotFound(
        device_descriptor.vendor_id(),
        device_descriptor.product_id(),
        format!("{:?}", transfer_type),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_device_not_found() {
        let context = Context::new().unwrap();
        let device = get_device(&context, 0, 0);
        assert!(device.is_err());
    }

    #[test]
    fn test_get_device() {
        let context = Context::new().unwrap();
        let device = get_device(&context, 0x0cf3, 0x3005);
        assert!(device.is_ok());
    }

    #[test]
    fn test_get_readable_endpoint() {
        let context = Context::new().unwrap();

        // readable device in my system, change these two values in your system
        let vendor_id = 0x0cf3;
        let product_id = 0x3005;
        let result = get_device(&context, vendor_id, product_id);
        assert!(result.is_ok());
        let (device, device_descriptor) = result.unwrap();
        let endpoint = get_readable_endpoint(
            &context,
            &device,
            &device_descriptor,
            TransferType::Interrupt,
        );
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
