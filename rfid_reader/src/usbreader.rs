pub(crate) trait UsbReader {
    fn read(&self) -> Box<[u8]>;
}
