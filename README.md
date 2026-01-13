To restore the Arduino bootloader:

```
RUST_LOG=info probe-rs download  --chip R7FA4M1AB --binary-format=ihex support/dfu_wifi.hex
```

To read OFS values in binary (the default linker script should omit these):

```
cargo ofsdump
```
