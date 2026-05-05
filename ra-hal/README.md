# ra-hal

ra-hal is a Hardware Abstraction Layer (HAL) for the Renesas [Renesas RA](https://www.renesas.com/en/products/microcontrollers-microprocessors/ra-cortex-m-mcus) family of microcontrollers written in Rust.
This HAL implements a variety of traits to provide both asynchronous and blocking interfaces that enable compatibility with software frameworks like [Embassy](https://embassy.dev) and [RTIC](https://rtic.rs/2/book/en/)
as well as more general libraries like [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/).
The end goal is to provide a safe, idiomatic Rust interface that surfaces most of the flexibility provided by the `RA` microcontrollers.

## Documentation

For more information check out the docs and take a peek at the [examples](../examples/).
To build documentation for a specific group run:

```console,ignore
just doc-<chip>
```

Replace `<chip>` with the model of your MCU.
To automatically open the generated documentation in a web browser append the `--open` argument.

## Application Template

A template suitable for [`cargo-generate`](https://crates.io/crates/cargo-generate) is available at: <https://git.sr.ht/~az1/ra-template>.
This template can be used to generate an Embassy project that uses the HAL or a PAC based project.

### Prerequisites

* Ensure [`cargo-generate`](https://crates.io/crates/cargo-generate) is installed.
* Get familiar with [Embassy](https://embassy.dev/).

### Usage

```console, ignore
$ cargo generate --git https://git.sr.ht/~az1/ra-template --branch metapac
<follow the prompts>
$ cd <name of your new project>
$ cargo build --release
$ cargo run --release
```

Check out the [examples](https://git.sr.ht/~az1/ra-hal/tree/master/item/ra-examples) and the [Embassy](https://embassy.dev) docs if you're not already familiar.

## Limitations

* Currently only one core per MCU is supported (relevant for multi-core RA8 MCUs). 
* On MCUs with TrustZone™ support the HAL is designed to only work in secure mode.
* `RA2`: Use of the `RA2` family is subject to significant caveats.  No compile- or run-time checking of interrupt groups is done.  Atomic ops may hang or report failure (TN-RA*-A0098A/E).
* `EK-RA2A1`: Despite a reported 500 kHz maximum SWO speed, the onboard J-Link seems to work most reliably with `probe-rs` when the SWO frequency is limited to 125 kHz via the `--speed 125` argument.
* TODO: Add knobs to generate a HAL+RTICv2 project.

For the canonical list of known issues check the issue tracker.

<https://todo.sr.ht/~az1/ra-rs/>

## Development

### Prerequisites

* MSRV is 1.95, `rust-toolchain.toml` has been set to 1.95.
  Older versions of Rust will not work as we depend on `cfg_select!`.
* [`just`](https://just.systems/man/en/). `just` can be installed by via `cargo install just`.
* [`probe-rs`](https://probe.rs) for debugging and flashing.

To use a local version of `ra-metapac` for development you can patch cargo with a configuration file named `cargo-config.toml` in the root directory of `ra-hal`.
For example:

```toml
[patch."https://git.sr.ht/~az1/ra-metapac"]
ra-metapac = { path = "/path/to/ra-metapac/ra-metapac" }
```
