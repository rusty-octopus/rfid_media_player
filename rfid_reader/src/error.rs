use crate::id::{ProductId, VendorId};
use libusb;
#[derive(Debug, PartialEq)]
pub enum Error {
    DeviceNotFound(VendorId, ProductId),
    LibUsbTimeout,
    LibUsbNoDevice,
    LibUsbNotFound,
    LibUsbSuccess,
    LibUsbIo,
    LibUsbInvalidParam,
    LibUsbAccess,
    LibUsbBusy,
    LibUsbOverflow,
    LibUsbPipe,
    LibUsbInterrupted,
    LibUsbNoMem,
    LibUsbNotSupported,
    LibUsbOther,
    ReadableInterruptEndPointNotFound(VendorId, ProductId),
    InvalidData,
    TooFewReceivedData,
    KeyNotExisting(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<libusb::Error> for Error {
    fn from(error: libusb::Error) -> Self {
        match error {
            libusb::Error::NoDevice => Self::LibUsbNoDevice,
            libusb::Error::Timeout => Self::LibUsbTimeout,
            libusb::Error::Access => Self::LibUsbAccess,
            libusb::Error::Busy => Self::LibUsbBusy,
            libusb::Error::Interrupted => Self::LibUsbInterrupted,
            libusb::Error::InvalidParam => Self::LibUsbInvalidParam,
            libusb::Error::Io => Self::LibUsbIo,
            libusb::Error::NoMem => Self::LibUsbNoMem,
            libusb::Error::NotFound => Self::LibUsbNotFound,
            libusb::Error::NotSupported => Self::LibUsbNotSupported,
            libusb::Error::Other => Self::LibUsbOther,
            libusb::Error::Success => Self::LibUsbSuccess,
            libusb::Error::Overflow => Self::LibUsbOverflow,
            libusb::Error::Pipe => Self::LibUsbPipe,
        }
    }
}
