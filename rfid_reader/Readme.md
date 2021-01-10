# Readme

* Crate for reading RFID values from RFID reader that behave like keyboards
* Crate does currently only support the Neuftech RFID Reader but could be easily extended or even used for different RFID readers

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
rfid_reader = "1.0.0"
```

Then use rfid_reader the following way:

```rust
use std::time::Duration;
use rfid_reader::{VendorId, ProductId, RfidReader};

// set Vendor Id and Product Id
let vendor_id = VendorId::from(0x16c0);
let product_id = ProductId::from(0x27db);

// Define a timeout (i.e. how long waits the reader for a RFID chip).
// From my experience, the longer the better, however it will block this amount of time.
let timeout = Duration::from_secs(60);
// get the rfid device, this may fail, for example when the device does not exist
let mut rfid_reader = rfid_reader::open(vendor_id, product_id, timeout).unwrap();

// Read from the device. This is blocking!
// It should normally return a String containing numbers, but it may also fail.
// It will return Error::Timeout whenever the timeout has expired
let data = rfid_reader.read().unwrap();

// Ideally de-initialize the device when no longer needed.
// This may fail therefore it makes sense to call it in order to get the error instead of
// a panic.

let result = rfid_reader.deinitialize();
if result.is_err() {
  println!("De-initialize failed: {}", result.unwrap_err());
}
```

## Release notes

* 1.0.0
  * First release version

## Code coverage

* See tarpaulin [HTML report](../tarpaulin-report.html)

## License

[MIT license](LICENSE).

## Design

```plantuml
package rfid_reader <<rectangle>> {
  class "lib.rs" as lib <<(L,lightpink)>>
  enum "error" as error_class
  interface humbleusbdevice <<(T,lightgreen)>>
  class "id" as id_class <<(S,lightskyblue)>>
  interface keymap <<(T,lightgreen)>>
  interface "rfidreader" as rfidreader_class <<(T,lightgreen)>>
  interface usbreader <<(S,lightskyblue)>>
  
  package neuftech <<rectangle>> {
    class "mod.rs" as neuftech_mod <<(M,orchid)>>
    class "keymap" as neuftech_keymap <<(S,lightskyblue)>>
    class "usbreader" as neuftech_usbreader <<(S,lightskyblue)>>
  }
  package rusb <<rectangle>> {
    class "mod.rs" as rusb_mod <<(M,orchid)>>
    class rusbhumbleusbdevice <<(S,lightskyblue)>>
    class utils <<(S,lightskyblue)>>
  }
}

lib -up-> error_class
lib -up-> id_class
lib -up-> rfidreader_class
lib -left-> keymap
lib -left-> usbreader
lib --> humbleusbdevice

lib ----> neuftech
lib ----> rusb

neuftech_mod -down-> neuftech_keymap
neuftech_mod -down-> neuftech_usbreader
neuftech_keymap -up.|> keymap
neuftech_usbreader -up.|> usbreader
neuftech_usbreader -up-> humbleusbdevice

rusb_mod -down-> utils
rusb_mod -down-> rusbhumbleusbdevice
rusbhumbleusbdevice -> utils
rusbhumbleusbdevice -up.|> humbleusbdevice
```

## Cross compilation

### aarch64-unknown-linux-musl

* Install target with `rustup`

```shell
rustup target add aarch64-unknown-linux-musl
```

* Install the GNU Compiler Collection - cross compiler for ARM64 target (below on ArchLinux)

```shell
sudo pacman -S aarch64-linux-gnu-gcc
```

* Install `musl-gcc`, in ArchLinux, this is contained in the `musl` package

```shell
sudo pacman -S musl
```

* Install musl kernel headers, in ArchLinux

```shell
sudo pacman -S kernel-headers-musl
```

* Build with setting environment `REALGCC` and `TARGET_CC`
  * See [Adventures in Rust and Cross Compilation for the Raspberry Pi](https://www.growse.com/2020/04/26/adventures-in-rust-and-cross-compilation-for-the-raspberry-pi.html)

```shell
REALGCC=aarch64-linux-gnu-gcc TARGET_CC=musl-gcc cargo build --target=aarch64-unknown-linux-musl
```

### aarch64-unknown-linux-gnu

* Works with [cross](crates.io/crates/cross) (no own Dockerfile needed)
