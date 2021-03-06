extern crate rusb;
use std::time::Duration;

#[cfg(not(tarpaulin_include))]
fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }

    let mut device_handle =
        rusb::open_device_with_vid_pid(0x16c0, 0x27db).expect("device not found");
    let active_configuration = device_handle.active_configuration();
    println!("Active configuraton: {:?}", active_configuration);
    //device_handle.reset().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    println!(
        "Has kernel driver: {:?}",
        device_handle.kernel_driver_active(0)
    );
    println!(
        "Detach kernel driver? {:?}",
        device_handle.detach_kernel_driver(0)
    );
    device_handle.claim_interface(0).unwrap();
    for _ in 0..15 {
        let read_result = device_handle.read_interrupt(0x81, &mut buffer, Duration::from_secs(10));
        if read_result.is_ok() {
            println!("Read result: {:?}", read_result);
            println!("Buffer: {:?}", buffer);
            let value = String::from_utf8(Vec::from(buffer));
            if value.is_ok() {
                let value = value.unwrap();
                println!("Value: {}", value);
            } else {
                let error = value.unwrap_err();
                println!("{:?}", error);
            }
        } else {
            println!(
                "Could not read from the device: {:?}",
                read_result.unwrap_err()
            );
        }
    }
    device_handle.attach_kernel_driver(0).unwrap();
}
