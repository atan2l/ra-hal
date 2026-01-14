To restore the Arduino bootloader:

```
RUST_LOG=info probe-rs download --chip R7FA4M1AB --binary-format=ihex support/dfu_wifi.hex
```

To read OFS values in binary (the default linker script should omit these):

```
cargo ofsdump
```

To build your own app:

Use `memory.x` and `build.rs` from the examples.
Note that you will have to specify `OFS1`, `OFS0`, and the Security MPU config and ensure they're in the proper section.
See examples for more details.
