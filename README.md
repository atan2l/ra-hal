# `ra-hal`
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![builds.sr.ht status](https://builds.sr.ht/~az1/ra-hal.svg)](https://builds.sr.ht/~az1/ra-hal?)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

This project contains Rust crates that provide Rust support for the [Renesas RA](https://www.renesas.com/en/products/microcontrollers-microprocessors/ra-cortex-m-mcus) family of microcontrollers.

## Contents
* [`ra-hal`](https://git.sr.ht/~az1/ra-hal/tree/master/item/ra-hal/README.md)
[![ra-hal crates.io](https://img.shields.io/crates/v/ra-hal)](https://crates.io/crates/ra-hal)
[![ra-hal docs.rs status](https://img.shields.io/docsrs/ra-hal/latest)](https://docs.rs/ra-hal)
* [`examples`](https://git.sr.ht/~az1/ra-hal/tree/master/item/examples)
Unpublished, see repo.
* [`uno-r4wifi-bsc`](https://git.sr.ht/~az1/ra-hal/tree/master/item/uno-r4wifi-bsc/README.md)
[![uno-r4wifi-bsc crates.io](https://img.shields.io/crates/v/uno-r4wifi-bsc)](https://crates.io/crates/uno-r4wifi-bsc)
[![uno-r4wifi-bsc docs.rs status](https://img.shields.io/docsrs/uno-r4wifi-bsc/latest)](https://docs.rs/uno-r4wifi-bsc)

## Requirements

* MSRV is 1.95, `rust-toolchain.toml` has been set to 1.95.
* [`just`](https://crates.io/crates/just). `just` can be installed by via `cargo install just`.
* [`probe-rs`](https://probe.rs) for debugging and flashing.

## Working with this repository

In order to apply a thin veneer of user friendliness and paper over some rough edges with `cargo`, this repository makes use of `just`.
Running `cargo` directly is generally discouraged.  To list available commands run `just` from the top level directory.

For more information about the HAL itself, see its [readme](https://git.sr.ht/~az1/ra-hal/tree/master/item/ra-hal/README.md).

### Examples

Examples are located in the [`examples/`](https://git.sr.ht/~az1/ra-hal/tree/master/item/examples) directory, they can be built and run with `just`.
For example:

```console,ignore
$ just examples ra4l1 run --release --bin=pulse
```

### Development

To use a local version of `ra-metapac` for development you can patch cargo with a configuration file named `cargo-config.toml` in the root directory of `ra-hal`.
For example:

```toml
[patch."https://git.sr.ht/~az1/ra-metapac"]
ra-metapac = { path = "/path/to/ra-metapac/ra-metapac" }
```

## Flashing

By default `cargo` generates an ELF binary that can be used with the deployment tool of your choice.
The typical workflow is to use `probe-rs` and set it as the runner for `cargo`.
If you're using `ra-template` or the examples here `probe-rs` is already configured.

For `probe-rs` versions prior to 0.32 you will need to manually specify the chip with the `--chip` argument.

### Special considerations with Arduino

The most common consumer boards to use these MCUs are Arduinos like the Uno R4 Minima and Uno R4 WiFi.
These crates *will* clobber the stock Arduino bootloader.
Note that the Uno R4 WiFi lacks a debug header but the included ESP32 chip acts as a CMSIS compliant debug probe in concert with the USB port.
Other boards will need a separate debug probe connected to the 10-pin debug header.

To restore the Arduino bootloader on an Uno R4 WiFi:

```
RUST_LOG=info probe-rs download --chip R7FA4M1AB --binary-format=ihex support/dfu_wifi.hex
```

To restore the bootloader on other other boards check the Arduino [repository](https://github.com/arduino/ArduinoCore-renesas/tree/main/bootloaders) for the suitable Intel Hex file and invoke `probe-rs` as above.

## Miscellaneous

### OFS
*TODO: mangle this into the Justfile*

To read OFS values from the binary (the default linker script should include these sections), use `cargo ofsdump`.
Note: `cargo-binutils` and `direnv` are required for the ofsdump plugin to work.

```
$ cargo ofsdump --bin=blink

blink:  file format elf32-littlearm
Contents of section .ofs0:
 0400 fffffbff                             ....
Contents of section .ofs1:
 0404 dfceffff                             ....
Contents of section .sec_mpu:
 0408 fcffff00 ffffff00 fcffff00 ffffff00  ................
 0418 fcffff00 ffffff00 fcff0f20 ffff0f20  ........... ...
 0428 fcff7f40 ffff7f40 fcff0d40 ffff0d40  ...@...@...@...@
 0438 ffffffff                             ....
```

To read the OFS values from a board connected to your computer:

```
./tools/decode-ofs.rb
```

## License

The examples are available under the terms of the GNU General Public License 3.0 (or newer).
All of the other code in this repository is available under the terms of the LGPL 3.0 (or newer) license.

Contributions to this project must be your own creation and made available under the CC0 license in order to be accepted.
Kindly exercise your prompting skills somewhere else.
