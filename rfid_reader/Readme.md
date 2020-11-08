# Readme

## Todos

* Implement Neuftech reader with as much as possible using libusbutils
  * Neuftech reader just stores COntext, Endpoint etc. but not Device, Device handle etc.
  * Device, DeviceHandle is opened on each Neuftech read, which are several reads on the USB devices
  * Implement Drop
* Fix Error
  * One global Error Type (besides Neuftech internal errors)
  * Handle libusb errors so that Error can be Debug, PartialEq, Display, etc.
  * RfidReader::read can fail
* Implement NewType Pattern: where appropriate
  * [NewType Pattern in Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)
* Clean-up
  * Documentation
  * Remove warnings
  * Lint
  * Test Coverage
  * Example / Integration test with actual Reader?
