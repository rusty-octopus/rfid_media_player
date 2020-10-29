use std::time::Duration;

pub(crate) enum UsbReaderError {
    DeviceNotFound(String),
}

struct DummyUsbReader;

impl UsbReader for DummyUsbReader {
    fn read(&self) -> Box<[u8]> {
        Box::from([0])
    }
}

pub(crate) fn open(
    vendor_id: u16,
    product_id: u16,
    timeout: Duration,
) -> Result<impl UsbReader, UsbReaderError> {
    Ok(DummyUsbReader)
}

pub(crate) trait UsbReader {
    fn read(&self) -> Box<[u8]>;
}
