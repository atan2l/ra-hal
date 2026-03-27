# `ra4m1-rs`
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![builds.sr.ht status](https://builds.sr.ht/~az1/ra4m1-rs.svg)](https://builds.sr.ht/~az1/ra4m1-rs?)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

This project contains Rust crates that provide Rust support for the [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) group of microcontrollers.
Some consideration has been given to eventually supporting other RA groups but for now regardless of the naming convention only the `RA4M1` is supported.

## Contents
* [`ra4m1-ctpac`](https://git.sr.ht/~az1/ra4m1-rs/tree/master/item/ra4m1-ctpac/README.md)  
[![ra4m1-ctpac crates.io](https://img.shields.io/crates/v/ra4m1-ctpac)](https://crates.io/crates/ra4m1-ctpac)
[![ra4m1-ctpac docs.rs status](https://img.shields.io/docsrs/ra4m1-ctpac/latest)](https://docs.rs/ra4m1-ctpac)
* [`ra4-hal`](https://git.sr.ht/~az1/ra4m1-rs/tree/master/item/ra4-hal/README.md)  
[![ra4-hal crates.io](https://img.shields.io/crates/v/ra4-hal)](https://crates.io/crates/ra4-hal)
[![ra4-hal docs.rs status](https://img.shields.io/docsrs/ra4-hal/latest)](https://docs.rs/ra4-hal)
* [`ra4-examples`](https://git.sr.ht/~az1/ra4m1-rs/tree/master/item/ra4-examples)  
Unpublished, see repo.
* [`uno-r4wifi-bsc`](https://git.sr.ht/~az1/ra4m1-rs/tree/master/item/uno-r4wifi-bsc/README.md)  
[![uno-r4wifi-bsc crates.io](https://img.shields.io/crates/v/uno-r4wifi-bsc)](https://crates.io/crates/uno-r4wifi-bsc)
[![uno-r4wifi-bsc docs.rs status](https://img.shields.io/docsrs/uno-r4wifi-bsc/latest)](https://docs.rs/uno-r4wifi-bsc)

## Requirements

MSRV is 1.91, `rust-toolchain.toml` has been set to 1.91.

## Building your own app

A template suitable for `cargo-generate` is available at: <https://git.sr.ht/~az1/ra4m1-template>.
For more information check out the docs for `ra4-hal` and take a peek at the examples.

## Flashing

The most common consumer boards to use this MCU are Arduinos like the Uno R4 Minima and Uno R4 WiFi.
These crates *will* clobber the stock Arduino bootloader.
Note that the Uno R4 WiFi lacks a debug header but the included ESP32 chip acts as a CMSIS compliant debug probe in concert with the USB port.
Other boards will need a separate debug probe connected to the 10-pin debug header.

To restore the Arduino bootloader on an Uno R4 WiFi:

```
RUST_LOG=info probe-rs download --chip R7FA4M1AB --binary-format=ihex support/dfu_wifi.hex
```

To restore the bootloader on other other boards check the Arduino [repository](https://github.com/arduino/ArduinoCore-renesas/tree/main/bootloaders) for the suitable Intel Hex file and invoke `probe-rs` as above.

## Miscellaneous 

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

## HAL Support

|            | Peripheral                                         | Notes                                                       |
| ---------- | -------------------------------------------------- | ----------------------------------------------------------- |
| ⬛⬛⬛⬜⬜ | `ADC14` 14-Bit A/D Converter                       | In progress, single shot for 1 and multi channel work.      |
| ⬜⬜⬜⬜⬜ | `CAC` Clock Frequency Accuracy Measurement Circuit | Internal oscillators aren't accurate enough to calibrate?   |
| ⬜⬜⬜⬜⬜ | `CAN` Controller Area Network                      |                                                             |
| ⬛⬛⬛⬛⬛ | `CRC` Cyclic Redundancy Check Calculator           | 32-bit CRC only works on multiples of 4 bytes.              |
| ⬛⬜⬜⬜⬜ | `DAC12` 12-bit D/A Converter                       | In progress.                                                |
| ⬛⬛⬛⬛⬛ | `FMIFRT` Factory MCU Information Flash Root Table  |                                                             |
| ⬛⬛⬛⬛⬜ | `GPT` General PWM Timer                            |                                                             |
| ⬛⬛⬛⬜⬜ | `IIC` I2C Bus                                      | In progress. Slave, MultiMaster, DMA, Error handling TODO.  |
| ⬛⬛⬛⬛⬜ | `IWDT` Independent Watchdog Timer                  | See WDT.                                                    |
| ⬛⬛⬛⬛⬜ | `PORT` I/O Ports                                   | In progress.                                                |
| ⬛⬛⬜⬜⬜ | `RTC` Real-Time Clock                              | In progress. Needs `CAC` for trimming.                      |
| ⬜⬜⬜⬜⬜ | `SCE5` Secure Cryptographic Engine                 | Undocumented.                                               |
| ⬛⬛⬛⬜⬜ | `SCI` Serial Communications Interface              | In progress, Embedded-serial pending.                       |
| ⬛⬛⬛⬜⬜ | `SPI` Serial Peripheral Interface                  | In progress. Blocking and DMAC via EH 1.0 works.            |
| ⬜⬜⬜⬜⬜ | `SSIE` Serial Sound Interface Enhanced             | Only on 100-pin variants.                                   |
| ⬛⬛⬛⬛⬜ | `WDT` Watchdog Timer                               | User defined handlers?                                      |

## License

The examples are available under the terms of the GNU General Public License 3.0 (or newer).
All of the other code in this repository is available under the terms of the LGPL 3.0 (or newer) license.
