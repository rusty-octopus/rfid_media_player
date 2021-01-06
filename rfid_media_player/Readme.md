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
