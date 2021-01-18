# Readme

* Application that can read RFID cards (TK4100, EM41000) and play tracks
* Tracks must be supplied in a simple key value yaml file (key=RFID value, value=path to track)
* Works with Neuftech USB RFID Reader ID
  * Should work with other USB RFID Readers as long as they act as a keyboard and provide the RFID value as a decimal string with an enter at the end
  * Vendor ID and Product ID must be provided to access the device (`lsusb` is of help here)
* The application opens the device, therefore it must either have root access or the device must allow opening from an unprivileged user (e.g. `chmod a+w /dev/bus/002/004`)

## Basic usage

```shell
rfid_media_player --product_id <PRODUCT_ID> --tracks <TRACKS_FILE> --vendor_id <VENDOR_ID>
```

* More information by calling:

```shell
rfid_media_player --help
```

## Issues

* Running the application as privileged user (e.g. using `sudo`) does stop the playback before the track is finished

## Enable access to USB device without root privilege

* Use [`50-usb-rfid-reader.rules`](50-usb-rfid-reader.rules) file and modify `ATTRS{idVendor}` and `ATTRS{idProduct}` accordingly
  * Alternatively use [`51-usb-rfid-reader.rules`](51-usb-rfid-reader.rules) (this one enables access for all users, so this is sub-optimal)
* Copy modified file to `/etc/udev/rules.d/` (may need root privilege)
* Unplug & plug USB RFID reader or execute `udevadm control --reload-rules` (may need root privilege)
* See [Ask Ubuntu - How do I make libusb work as non root](https://askubuntu.com/questions/978552/how-do-i-make-libusb-work-as-non-root)

## Systemd user service usage

* Prerequisites: USB RFID reader device must be accessible from your user, see [Enable access to USB device without root privilege](#enable-access-to-usb-device-without-root-privilege)
* Modify [`rfid_media_player.service`](rfid_media_player.service) file
* Copy the modified file to `~/.config/systemd/user`
* Start the service with `systemctl --user start rfid_media_player.service`
* Start the service with `systemctl --user stop rfid_media_player.service`
* Enable the service on start-up with `systemctl --user enable rfid_media_player.service`
* See [ArchLinux Wiki - Systemd/User](https://wiki.archlinux.org/index.php/Systemd/User)

## Release notes

* 1.0.0
  * First release version

## Code coverage

* See [tarpaulin-report](../tarpaulin-report)

## License

[MIT license](LICENSE).

## Cross compilation

### Using cross

* Does not work, since workspace dependencies cannot be found (and I don't know how to add these in cross)

### aarch64-unknown-linux-musl

* Does not work, since [alsa-sys](https://github.com/diwic/alsa-sys/) links dynamically to alsa (dependency of [alsa](crates.io/crates/alsa) which is a dependency of [rodio](crates.io/crates/rodio)) which is a dependency of [media_player](../media_player)
