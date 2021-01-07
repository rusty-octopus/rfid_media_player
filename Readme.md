# Readme

* Cargo workspace of rfid_media_player application

## Relevant Crates

* [rfid_media_player](./rfid_media_player/Readme.md): The application that plays a track when a RFID value is read
* [rfid_reader](./rfid_reader/Readme.md): Library that handles with the USB RFID reader
* [media_player](./media_player/Readme.md): Library that handles the playing of the tracks
* [track_store](./track_store/Readme.md): Library that stores the RFID values as keys and the paths to the tracks as values

## Example crates

* [usb_reader_test](usb_reader_test/Readme.md): Simple application build in order to learn how to access the USB device
* [media_player_test](media_player_test/Readme.md): Simple application build in order to learn how to use audio library

## Todos

1. Consider implementing read result as RfidValue NewType
2. Debug running as root problem
3. Cross Compile + documentation in crates + here
4. Documentation here
5. License analysis dependencies: Either MIT or Apache 2.0 from all direct deps

## Useful Links

* [Compile Rust for Raspberry Pi ARM](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050)
* [Cli Handbook](https://rust-cli.github.io/book/index.html)
  * [Signal handling](https://rust-cli.github.io/book/in-depth/signals.html)
* [Cross compile RaspPI](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/)
* [RaspPI OS tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
* [USB keyboard codes](https://www.win.tue.nl/~aeb/linux/kbd/scancodes-14.html)
* [cross](https://crates.io/crates/cross)
* [Cross compiling rust apps raspberry pi](https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/)
* [Adventures in Rust and Cross compilation for RP](https://www.growse.com/2020/04/26/adventures-in-rust-and-cross-compilation-for-the-raspberry-pi.html)

## Architecture

### Component

```plantuml
component rfid_media_player
component media_player
component rfid_reader
component tracks
component "std::fs" as file #Crimson
component signal_hook #LightGrey
component log #LightGrey
component flexi_logger #LightGrey
component rusb #LightGrey
component rodio #LightGrey
component serde #LightGrey
component serde_yaml #LightGrey
component clap #LightGrey

rfid_media_player -down-> rfid_reader
rfid_media_player -down-> media_player
rfid_media_player -down-> file
rfid_media_player -down-> signal_hook
rfid_media_player -down-> tracks
rfid_media_player -down-> clap
rfid_media_player -down-> log
rfid_media_player -down-> flexi_logger
media_player -down-> rodio
media_player -left-> file
rfid_reader -down-> rusb
tracks -down-> serde
tracks -down-> serde_yaml

```

### Sequence

#### Play new song with idle player

```plantuml
actor user
participant rfid_hw
participant rfid_media_player
participant rfid_reader
participant track_store
participant "std::fs" as file #Crimson
participant media_player

user --> rfid_hw: places card
rfid_hw --> rfid_reader: interrupt read
rfid_reader --> rfid_media_player: rfid number
rfid_media_player -> track_store: get path
track_store --> rfid_media_player
rfid_media_player -> media_player: play
media_player -> media_player: still playing?:false 
media_player -> file: read file
file --> media_player
media_player -> media_player: play track
note over media_player
song is played in background
end note
media_player --> rfid_media_player
rfid_media_player -> rfid_reader: re-activate & wait
```

#### Play new song with busy player

```plantuml
actor user
participant rfid_hw
participant rfid_media_player
participant rfid_reader
participant track_store
participant file #Crimson
participant media_player

user --> rfid_hw: places card
rfid_hw --> rfid_reader: interrupt read
rfid_reader --> rfid_media_player: rfid number
rfid_media_player -> track_store: get path
track_store --> rfid_media_player
rfid_media_player -> media_player: play
media_player -> media_player: still playing?:true
media_player -> media_player: check new track!=old track
alt old track != new track
  media_player -> file: read file
  file --> media_player
  media_player -> media_player: stop current song
  media_player -> media_player: play song
end
media_player --> rfid_media_player
rfid_media_player -> rfid_reader: re-activate & wait
```
