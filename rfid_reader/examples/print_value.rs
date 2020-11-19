use rfid_reader::{open, Error, ProductId, RfidReader, VendorId};
use std::time::Duration;

fn main() {
    let vendor_id = VendorId::from(0x16c0);
    let product_if = ProductId::from(0x27db);
    let reader = open(vendor_id, product_if, Duration::from_secs(60));

    if reader.is_ok() {
        let reader = reader.unwrap();
        loop {
            let read_result = reader.read();
            match read_result {
                Ok(value) => println!("Value: {}", value),
                Err(error) => println!("Error: {}", error),
            }
        }
    } else {
        println!("Error: {:?}", reader.unwrap_err());
    }
}
