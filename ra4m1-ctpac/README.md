# `ra4m1-ctpac`

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for Renesas RA4M1 microcontrollers.

The crate itself is generated programmatically by [chiptool](https://github.com/embassy-rs/chiptool) using the SVD as found on Keil's [site](https://www.keil.arm.com/packs/ra_dfp-renesas/versions/).

## Supported Hardware

In the consumer space the Renesas RA4M1 is most commonly found in the Aduino "R4" boards like the Uno R4 Minima and Nano R4.

## TODO

* Get required PRs merged into chiptool.
* Continue renaming enum variants with symbolic names.
* Add transforms any additional missing chunks.

## Rebuilding

To regenerate the Rust code run the `generate.sh` script from this directory.
Note: that this crate currently **cannot** be rebuilt with the stock chiptool.
