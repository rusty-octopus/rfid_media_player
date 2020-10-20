extern crate libusb;
use std::time::Duration;

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
    }

    let mut device_handle = context.open_device_with_vid_pid(0x16c0, 0x27db).expect("device not found");
    let active_configuration = device_handle.active_configuration();
    println!("Active configuraton: {:?}", active_configuration);
    //device_handle.reset().unwrap();
    //device_handle.claim_interface(0).unwrap();
    let mut buffer:[u8;100] = [0;100];
    let read_result = device_handle.read_interrupt(0x81, & mut buffer, Duration::from_secs(10));
    if read_result.is_ok() {
      let value = String::from_utf8(Vec::from(buffer));
      if value.is_ok() {
        let value = value.unwrap();
      println!("Value: {}", value);
      } else {
        let error = value.unwrap_err();
        println!("{:?}", error);
      }
    } else {
      println!("Could not read from the device: {:?}", read_result.unwrap_err());
    }
}
