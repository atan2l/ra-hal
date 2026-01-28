# `ra4m1-rs`

This repository contains Rust crates that provide Rust support for the [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) group of microcontrollers.

## Requirements

MSRV is 1.91, `rust-toolchain.toml` has been set to 1.91.

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

Uno R4 WiFi = R7FA4M1AB3CFM#AA0 = 64 pin LQFP

To read OFS values in binary (the default linker script should include these):

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

## Building your own app

Use `memory.x` and `build.rs` from the `ra4-examples` directory.
Note that you will have to specify `OFS1`, `OFS0`, and the Security MPU config and ensure they're in the proper section.
See examples for more details.

## HAL Support

|            | Peripheral                                         | Notes                                                  |
| ---------- | -------------------------------------------------- | ------------------------------------------------------ |
| ⬛⬛⬜⬜⬜ | `ADC14` 14-Bit A/D Converter                       | In progress, single shot readings work                 |
| ⬜⬜⬜⬜⬜ | `CAC` Clock Frequency Accuracy Measurement Circuit |                                                        |
| ⬛⬛⬛⬛⬛ | `CRC` Cyclic Redundancy Check Calculator           | 32-bit CRC only works on multiples of 4 bytes.         |
| ⬛⬛⬛⬛⬛ | `FMIFRT` Factory MCU Information Flash Root Table  |                                                        |
| ⬛⬜⬜⬜⬜ | `GPT` General PWM Timer                            | Embassy timer driver only, general PWM support pending |
| ⬛⬜⬜⬜⬜ | `IIC` I2C Bus                                      | In progress                                            |
| ⬛⬛⬛⬜⬜ | `PORT` I/O Ports                                   | In progress                                            |
| ⬛⬛⬜⬜⬜ | `RTC` Real-Time Clock                              | In progress. Needs `CAC` for trimming.                 |
| ⬜⬜⬜⬜⬜ | `SCE5` Secure Cryptographic Engine                 | Undocumented                                           |
| ⬛⬛⬛⬜⬜ | `SCI` Serial Communications Interface              | In progress, DMA (DTC) no-go, embedded-serial pending. |

## TODO

* App template
* Add another layer of macro goodness on top of the option setting registers
* Add spell checking to the build manifest.  <https://github.com/blopker/codebook/issues/39>

## License

All of the code in this repository is available under the terms of the LGPL 3.0 (or newer) license.
