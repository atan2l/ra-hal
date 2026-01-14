# `ra4m1`

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

To read OFS values in binary (the default linker script should omit these):

```
cargo ofsdump
```

## Building your own app:

Use `memory.x` and `build.rs` from the examples.
Note that you will have to specify `OFS1`, `OFS0`, and the Security MPU config and ensure they're in the proper section.
See examples for more details.

## Supported peripherals:

| Column 1      | Column 2      |
| ------------- | ------------- |
| **￭￭･･**      | fjjskfs       |

`⬤○`
`◼▭`
`￭￭･･`

* General PWM Timer (`GPT`): ◖ Embassy timer driver only, general PWM support pending
* Cyclic Redundancy Check Calculator (`CRC)`: ●

## TODO

App template
