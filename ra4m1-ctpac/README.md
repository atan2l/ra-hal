# `ra4m1-ctpac`

`ra4m1-ctpac` is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) microcontrollers.
Unlike similar PACs for the RA4M1, this crate is generated programmatically by [chiptool](https://github.com/embassy-rs/chiptool) using the SVD as found on Keil's [site](https://www.keil.arm.com/packs/ra_dfp-renesas/versions/).
The resulting API is different from other crates that are generated using svd2rust and the chiptool documentation covers these differences.
This is primarily intended to be used by an upcoming RA4M1 HAL crate, however a secondary goal is to make this a bit more legible to humans.

## Supported Hardware

In the consumer space the Renesas RA4M1 is most commonly found in the Arduino "R4" boards like the Uno R4 Minima and Nano R4.

## Updating

To update the Rust code run the `generate.sh` script from this directory.
`generate.sh` depends on chiptool, form, and GNU sed.
It will install and update chiptool via cargo unless `CHIPTOOL_CMD` is set to a valid executable.
Form will be installed and locked to a specific version via cargo if needed.

If you wish to use a different SVD, place it in the `support/svd` directory and set the `SVD` environment variable appropriately.

Note: that this crate currently **cannot** be rebuilt with the stock chiptool.

## TODO

* Get required PRs merged into chiptool.
* Continue renaming enum variants with symbolic names.
* Add transforms any additional missing chunks.
