# Readme

## Todos

* USB Reader as Humble Object?
  * [UI Archs](https://martinfowler.com/eaaDev/uiArchs.html)
  * Gerard Meszaros<http://xunitpatterns.com> generalized this notion to idea of a Humble Object - any object that is difficult to test should have minimal behavior."
  * [Test double](https://martinfowler.com/bliki/TestDouble.html)
* Reconsider Implementation of Drop, seem unsafe since it can panic
* Fix Lib Usb Error Mess
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
