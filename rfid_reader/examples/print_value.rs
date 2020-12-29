use rfid_reader::{open, Error, ProductId, RfidReader, VendorId};
use std::time::Duration;

fn main() {
    let vendor_id = VendorId::from(0x16c0);
    let product_if = ProductId::from(0x27db);
    let reader = open(vendor_id, product_if, Duration::from_secs(2));

    if reader.is_ok() {
        let mut reader = reader.unwrap();
        let mut counter = 0;
        while counter < 5 {
            counter += 1;
            let read_result = reader.read();
            match read_result {
                Ok(value) => println!("Value: {}", value),
                Err(error) => println!("Read Error: {}", error),
            }
        }
        reader.deinitialize();
    } else {
        println!(
            "Reader could not be opened. Error: {:?}",
            reader.unwrap_err()
        );
    }
}
