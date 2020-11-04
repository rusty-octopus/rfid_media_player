use libusb;
#[derive(Debug)]
pub enum Error {
    DeviceNotFound(u16, u16),
    LibUsbNoDevice,
    LibUsbAccess,
    LibUsbNotFound,
    LibUsbOther(libusb::Error),
    ReadableInterruptEndPointNotFound(u16, u16),
}

impl From<libusb::Error> for Error {
    fn from(error: libusb::Error) -> Self {
        match error {
            libusb::Error::NoDevice => Self::LibUsbAccess,
            _ => Self::LibUsbOther(error),
        }
    }
}
