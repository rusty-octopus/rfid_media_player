# Readme

## Open questions

* Does a long timeout cost CPU cycles?
  * Short showed: No!

## Device

* Bus 002 Device 003 ID 16c0:27db

## Deps

* [libusb](http://dcuddeback.github.io/libusb-rs/libusb/)

## Behavior

* Reads card number as keyboard input including a new line, e.g.:
  * `0013110988` <= with new line

## Links

* [lib usb examples](https://github.com/dcuddeback/libusb-rs/blob/master/examples/read_device.rs)
* [IOT Tutorial: Read RFID-tags with an USB RFID reader, Raspberry Pi and Node-RED from scratch](https://medium.com/coinmonks/iot-tutorial-read-tags-from-a-usb-rfid-reader-with-raspberry-pi-and-node-red-from-scratch-4554836be127)
* [UTF8 table](https://www.utf8-chartable.de/)
* [libusb examples](https://github.com/libusb/libusb/tree/master/examples)
* [USB in a nutshell](https://beyondlogic.org/usbnutshell/usb4.shtml)
