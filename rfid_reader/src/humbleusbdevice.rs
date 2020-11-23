#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;

pub trait HumbleUsbDevice {
    fn has_attached_kernel_driver(&self) -> Result<bool, Error>;
    fn detach_kernel_driver(&mut self) -> Result<(), Error>;
    fn attach_kernel_driver(&mut self) -> Result<(), Error>;
    fn read(&self, buffer: &mut [u8]) -> Result<(), Error>;
    fn claim_interface(&mut self) -> Result<(), Error>;
    fn release_interface(&mut self) -> Result<(), Error>;
    fn set_active_configuration(&mut self) -> Result<(), Error>;
    fn set_alternate_setting(&mut self) -> Result<(), Error>;
    fn initialize(&mut self) -> Result<(), Error> {
        if self.has_attached_kernel_driver()? {
            self.detach_kernel_driver()?;
            self.set_active_configuration()?;
            self.claim_interface()?;
            self.set_alternate_setting()?;
        }
        Ok(())
    }
    fn set_deinitialized(&mut self);
    fn deinitialized(&self) -> bool;
    fn deinitialize(&mut self) -> Result<(), Error> {
        if !self.deinitialized() {
            self.set_deinitialized();
            if self.has_attached_kernel_driver()? {
                self.attach_kernel_driver()?;
                self.release_interface()?;
            }
        }
        Ok(())
    }
}
