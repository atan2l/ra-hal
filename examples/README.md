Examples that demonstrate the `ra-hal` crate.

The easiest way to build is via [`just`](https://just.systems/man/en/).

### Running from this directory

```console,ignore
just ra4m1 build --release
```

Replace `ra4m1` with the chip you're interested in.

### Running from the repository root

```console,ignore
just examples ra4m1 build --release
```

Replace `ra4m1` with the chip you're interested in.

## Supported Boards

The examples must be configured for your specific board as this will determine which features and pins are available.
To change the board selection you can change the default features in `<chip>/Cargo.toml`.
Look for the line beginning with `default = [` and replace the board feature with the board you're using.

RA2A1:
* ek-ra2a1

RA4L1 (and RA4L1 RTIC examples):
* ek-ra4l1

RA4M1:
* uno-r4-minima 
* uno-r4-wifi 
* ek-ra4m1

RA6M5:
* ek-ra6m5

RA8M1:
* ex-ra8m1
