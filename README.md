# `ra4m1-rs`

This repository contains Rust crates that implement a variety of Hardware Abstraction Layer (HAL) traits to support the [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) microcontrollers.
These traits provide both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev).

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
cargo ofsdump
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

|            | Peripheral                                        | Notes                                                  |
| ---------- | ------------------------------------------------- | ------------------------------------------------------ |
| ⬛⬛⬜⬜⬜ | `ADC14` 14-Bit A/D Converter                      | In progress, single shot readings work                 |
| ⬛⬛⬛⬛⬛ | `CRC` Cyclic Redundancy Check Calculator          | 32-bit CRC only works on multiples of 4 bytes.         |
| ⬛⬛⬛⬛⬛ | `FMIFRT` Factory MCU Information Flash Root Table |                                                        |
| ⬛⬜⬜⬜⬜ | `GPT` General PWM Timer                           | Embassy timer driver only, general PWM support pending |
| ⬜⬜⬜⬜⬜ | `SCE5` Secure Cryptographic Engine                | Undocumented                                           |
| ⬛⬜⬜⬜⬜ | `SCI` Serial Communications Interface             | In progress                                            |

## TODO

* App template
* Add another layer of macro goodness on top of the option setting registers

## License

All of the code in this repository is available under the terms of the LGPL 3.0 (or newer) license.
