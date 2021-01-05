# Coredump analysis

## Reading core dump files on ArchLinux

### Using coredeumpctl & gdb

* You can use `coredumpctl gdb *match*` but this is not sufficient
* You need `~/.gdbinit` file with the following content:

```shell
add-auto-load-safe-path /home/<your user name>/.rustup/toolchains
dir /home/<your user name>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/
```

* Additionally you must patch the `gdb_load_rust_pretty_printers.py` file in the folder added to your `~/.gdbinit`. Just add the following lines before any other code:

```python
import sys
from os import path
self_dir = path.dirname(path.realpath(__file__))
sys.path.append(self_dir)
```

* See [Rust issue 33159](https://github.com/rust-lang/rust/issues/33159)

### Using coredumptctl & rust-gdb

* You need to dump the core dump into a file first `coredumpctl dump *match* > core_dump`
* Then you can use `rust-gdb <path to your rust binary> <path to your core_dump file>`
