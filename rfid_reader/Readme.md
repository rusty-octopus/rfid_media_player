# Readme

## Todos

* humbleusbdevice is extended to support more methods but does only define the traits
* New mod named rusb
    * RusbHumbleUsbDevice implementation
    * rusbutils
* neuftech/usbreader does only use a HumbleUsbDevice and implements all the logic to make the device available etc.
* Test against a dummy device
* USB Reader as Humble Object?
  * [UI Archs](https://martinfowler.com/eaaDev/uiArchs.html)
  * Gerard Meszaros<http://xunitpatterns.com> generalized this notion to idea of a Humble Object - any object that is difficult to test should have minimal behavior."
  * [Test double](https://martinfowler.com/bliki/TestDouble.html)
* Reconsider Implementation of Drop, seem unsafe since it can panic
* Use less allocations?
* Fix Error
  * One global Error Type (besides Neuftech internal errors)
  * Handle libusb errors so that Error can be Debug, PartialEq, Display, etc.
  * RfidReader::read can fail
* Clean-up
  * Documentation
  * Remove warnings
  * Lint
  * Test Coverage
  * Example / Integration test with actual Reader?
* Cross compile
  * [cross compile lib usb](https://github.com/dcuddeback/libusb-sys#cross-compiling)
