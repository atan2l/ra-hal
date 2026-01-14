# `ra4m1`

This repository contains crates that implement a variety of Hardware Abstraction Layer (HAL) to support the Renesas RA4M1 microcontrollers.
These traits implement both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev).


## Requirements

MSRV is 1.91, `rust-toolchain.toml` has been set to 1.91.

## Flashing

The most common boards to use this MCU are Arduinos like the Uno R4 Minima and Uno R4 WiFi.
These crates *will* clobber the stock Arduino bootloader.

To restore the Arduino bootloader on an Uno R4 WiFi:

```
RUST_LOG=info probe-rs download --chip R7FA4M1AB --binary-format=ihex support/dfu_wifi.hex
```

For other boards check the Arduino [repository](https://github.com/arduino/ArduinoCore-renesas/tree/main/bootloaders) for the suitable Intel Hex file.

## Miscellaneous 

To read OFS values in binary (the default linker script should omit these):

```
cargo ofsdump
```

## Building your own app:

Use `memory.x` and `build.rs` from the examples.
Note that you will have to specify `OFS1`, `OFS0`, and the Security MPU config and ensure they're in the proper section.
See examples for more details.


## TODO

App template
