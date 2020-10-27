use std::time::Duration;

pub(crate) enum UsbReaderError {
  DeviceNotFound(String),
}

pub(crate) trait UsbReader {
  //fn open(vendor_id:u16, product_id:u16, timeout:Duration) -> Result<Self, UsbReaderError> where Self: Sized;
  fn read(&self) -> [u8];
}