# Readme

## Links

* [Compile Rust for Raspberry Pi ARM](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050)
* [Cli Handbook](https://rust-cli.github.io/book/index.html)

## Useful crates

* [rodio](https://docs.rs/rodio/0.12.0/rodio/)
  * [Sink](https://docs.rs/rodio/0.12.0/rodio/struct.Sink.html)
* [Cross compile RaspPI](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/)
* [RaspPI OS tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
* [USB keyboard codes](https://www.win.tue.nl/~aeb/linux/kbd/scancodes-14.html)

## Architecture

### Component

```plantuml
component application
component media_player
component rfid_reader
component data_base
component file #Crimson

application -down-> rfid_reader
application -down-> data_base
application -down-> media_player
application -down-> file
```

### Sequence

#### Play new song with idle player

```plantuml
actor user
participant rfid_hw
participant application
participant rfid_reader
participant data_base
participant file #Crimson
participant media_player

user --> rfid_hw: places card
rfid_hw --> rfid_reader: interrupt read
rfid_reader --> application: rfid number
application -> media_player: still playing
media_player --> application: false 
application -> data_base: get path
data_base --> application
application -> file: read file
file --> application
application -> media_player: play song
note over media_player
song is played in background
end note
media_player --> application
application -> rfid_reader: re-activate & wait
```

#### Play new song with busy player

```plantuml
actor user
participant rfid_hw
participant application
participant rfid_reader
participant data_base
participant file #Crimson
participant media_player

user --> rfid_hw: places card
rfid_hw --> rfid_reader: interrupt read
rfid_reader --> application: rfid number
application -> media_player: still playing
media_player --> application: true
application -> application: check old number=new number
alt old number != new number
  application -> data_base: get path
  data_base --> application
  application -> file: read file
  file --> application
  application -> media_player: stop current song
  media_player --> application
  application -> media_player: play song
  media_player --> application
end
application -> rfid_reader: re-activate & wait
```

### rfid_reader

```plantuml
class RfidReader {
  + open(): Result<RfidReader>
  + read(): Future<String>
  + drop()
}

interface KeyMap {
  + map(u8):Result<char>
}
class NeuftechKeyMap


interface UsbReadDevice {
  + open(vendor_id, product_id):Result<UsbDevice>
  + read(): Future<[u8]>
  + drop()
}

class NeuftechUsbReadDevice

note left of NeuftechUsbReadDevice
  handles "protocol"
  returns only array with
  u8 values that represent keys
end note

class TimeoutHandler

class libusb #LightGrey

NeuftechUsbReadDevice -up-|> UsbReadDevice

RfidReader -down-> UsbReadDevice
RfidReader -down-> KeyMap

NeuftechKeyMap .up-|> KeyMap

NeuftechUsbReadDevice --> TimeoutHandler
NeuftechUsbReadDevice -down-> libusb
```
