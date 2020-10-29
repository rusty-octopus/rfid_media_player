use crate::usbreader::{UsbReader, UsbReaderError};

pub(crate) struct NeuftechUsbReader;

impl UsbReader for NeuftechUsbReader {
    fn read(&self) -> Box<[u8]> {
        let data = (0..10).collect::<Vec<u8>>().into_boxed_slice();
        data
    }
}
