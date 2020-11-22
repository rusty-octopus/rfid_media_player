#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::id::{ProductId, VendorId};
use rusb::{Device, DeviceDescriptor, Direction, TransferType, UsbContext};

#[derive(Debug, PartialEq)]
pub(crate) struct EndPoint {
    config: u8,
    interface: u8,
    setting: u8,
    address: u8,
}

impl EndPoint {
    pub(crate) fn get_interface(&self) -> u8 {
        self.interface
    }
    pub(crate) fn get_address(&self) -> u8 {
        self.address
    }
    pub(crate) fn get_config(&self) -> u8 {
        self.config
    }
    pub(crate) fn get_setting(&self) -> u8 {
        self.setting
    }
}

pub(crate) fn get_device<T: UsbContext>(
    context: &T,
    vendor_id: VendorId,
    product_id: ProductId,
) -> Result<(Device<T>, DeviceDescriptor), Error> {
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

pub(crate) fn get_readable_endpoint<T: UsbContext>(
    device: &Device<T>,
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
    Err(Error::ReadableEndPointNotFound(
        device_descriptor.vendor_id().into(),
        device_descriptor.product_id().into(),
    ))
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    #[test]
    fn test_get_device_not_found() {
        let context = rusb::Context::new().unwrap();
        let device = get_device(&context, VendorId::from(0), ProductId::from(0));
        assert!(device.is_err());
    }

    #[test]
    fn test_get_device() {
        // readable device in my system, change these two values in your system
        let vendor_id = VendorId::from(0x0cf3);
        let product_id = ProductId::from(0x3005);

        let context = rusb::Context::new().unwrap();
        let device = get_device(&context, vendor_id, product_id);

        assert!(device.is_ok());
    }

    #[test]
    fn test_get_readable_endpoint() {
        let context = rusb::Context::new().unwrap();

        // readable device in my system, change these two values in your system
        let vendor_id = VendorId::from(0x0cf3);
        let product_id = ProductId::from(0x3005);
        let result = get_device(&context, vendor_id, product_id);
        assert!(result.is_ok());
        let (device, device_descriptor) = result.unwrap();
        let endpoint = get_readable_endpoint(&device, &device_descriptor, TransferType::Interrupt);
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

    #[test]
    fn test_get_no_readable_endpoint() {
        let context = rusb::Context::new().unwrap();

        // device in my system without readable bulk endpoint, change these two values in your system
        let vendor_id = VendorId::from(0x058f);
        let product_id = ProductId::from(0xa004);
        let result = get_device(&context, vendor_id, product_id);
        assert!(result.is_ok());
        let (device, device_descriptor) = result.unwrap();
        let endpoint = get_readable_endpoint(&device, &device_descriptor, TransferType::Bulk);
        assert_eq!(
            Err(Error::ReadableEndPointNotFound(vendor_id, product_id)),
            endpoint
        );
    }

    #[test]
    fn test_endpoint() {
        let endpoint = EndPoint {
            config: 0,
            interface: 1,
            setting: 2,
            address: 3,
        };
        assert_eq!(0, endpoint.get_config());
        assert_eq!(1, endpoint.get_interface());
        assert_eq!(2, endpoint.get_setting());
        assert_eq!(3, endpoint.get_address());
    }
}
