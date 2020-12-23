# Readme

* Crate to play (audio) tracks
* Hides the actual audio library inside
* Currently uses [rodio](https://crates.io/crates/rodio)

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
media_player = "1.0.0"
```

Then use media_player the following way:

```rust
use media_player::{open, MediaPlayer, Track};

// get media_player trait object
let mut media_player = open().unwrap();

// create Track from String or &str
let track = Track::from("tests/rand1.wav");

// play the track
media_player.play(&track).unwrap();

// ...

// if you play a new track, the old one is stopped
let track2 = Track::from("tests/rand2.wav");
media_player.play(&track2).unwrap();

// ...

// you can also stop the playing of the track
media_player.stop().unwrap();
```

## Release notes

* 1.0.0
  * First release version

## Code coverage

* See tarpaulin [HTML report](../tarpaulin-report.html)

## License

[MIT license](LICENSE).
