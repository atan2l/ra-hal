# ra-hal

ra-hal is a Hardware Abstraction Layer (HAL) for the Renesas [Renesas RA](https://www.renesas.com/en/products/microcontrollers-microprocessors/ra-cortex-m-mcus) family of microcontrollers.
This HAL implements a variety of traits to provide both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev) and [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/).
The end goal is to provide a safe, idiomatic Rust interface that surfaces most of the flexibility provided by the `RA` microcontrollers.

## Application Template

A template suitable for [`cargo-generate`](https://crates.io/crates/cargo-generate) is available at: <https://git.sr.ht/~az1/ra-template>.

For more information check out the docs and take a peek at the examples.
To build documentation for a specific group run `just doc-<chip>` e.g. `just doc-ra4l1`.
To automatically open the generated documentation in a web browser append the `--open` argument.

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

* Currently only one core per MCU is supported, but there's no RA8 support yet so this is moot.
* On MCUs with TrustZone support the HAL is designed to only work in secure mode.
* `RA2`: Use of the `RA2` family is subject to significant caveats.  No compile- or run-time checking of interrupt groups is done.  Atomic ops may hang or report failure (TN-RA*-A0098A/E).
* `EK-RA2A1`: Despite a reported 500 kHz maximum SWO speed, the onboard J-Link seems to work most reliably with `probe-rs` when the SWO frequency is limited to 125 kHz via the `--speed 125` argument.

For the canonical list of known issues check the issue tracker.

<https://todo.sr.ht/~az1/ra-rs/>

## Development

The current MSRV is 1.95.
Older versions of Rust will not work as we depend on `cfg_select!`.
