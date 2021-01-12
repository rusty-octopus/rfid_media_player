# Readme

* Crate for a track store that enables getting the path of a track by its id.

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
track_store = "1.0.0"
```

Then use track_store the following way:

```rust
// use crate
use track_store::{load, TrackStore, Id, TrackPath};

/// simple key value list in the yaml string
let yaml_string = "01234: path/to/track";

/// load the TrackStore from the yaml string.
let track_store = load(yaml_string).unwrap();

/// Create an Id from a string
let id = Id::from(String::from("01234"));

/// get the track path as an Option
let track_path = track_store.get_path(&id);
assert!(track_path.is_some());
let expected_path = String::from("path/to/track").into();
assert_eq!(Some(&expected_path), track_path);

/// If the Id does not exist, the returned track path is None.
let id = Id::from(String::from("0"));
let track_path = track_store.get_path(&id);
assert!(track_path.is_none());
```

## Release notes

* 1.0.0
  * First release version

## Code coverage

* See [tarpaulin-report](../tarpaulin-report)

## License

[MIT license](LICENSE).

## Cross compilation

### armv7-unknown-linux-musleabihf

* Install target with `rustup`

```shell
rustup target add armv7-unknown-linux-musleabihf
```

* Build with target

```shell
cargo build --target=armv7-unknown-linux-musleabihf
```

### aarch64-unknown-linux-musl

* Install target with `rustup`

```shell
rustup target add aarch64-unknown-linux-musl
```

* Build with target

```shell
cargo build --target=aarch64-unknown-linux-musl
```

### aarch64-unknown-linux-gnu

* Works with [cross](crates.io/crates/cross) (no own Dockerfile needed)
