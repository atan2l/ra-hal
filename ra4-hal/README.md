# ra4-hal

ra4-hal is a Hardware Abstraction Layer (HAL) for the Renesas [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) group of microcontrollers.
This HAL implements a variety of traits to provide both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev) and [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/).
The end goal is to provide a safe, idiomatic Rust interface that surfaces most of the flexibility provided by the `RA4M1`.

## Usage

Prerequisites:

* Ensure [`probe-rs`](https://probe.rs) is installed.
* Ensure [`rustup`](https://rustup.rs/) is installed.
* Ensure [`cargo-generate`](https://crates.io/crates/cargo-generate) is installed if you plan to use the application template.
* Get familiar with [Embassy](https://embassy.dev/).

To get started with the app template:

```ignore
$ cargo generate --git https://git.sr.ht/~az1/ra4m1-template
<follow the prompts>
$ cd <name of your new project>
$ cargo build --release
$ cargo run --release
```

CliffsNotes™:
* Create a new Rust bin project
* Set the target arch to `thumbv7em-none-eabihf` in `.cargo/config.toml`
* Set the runner to something reasonable like `probe-rs` in `.cargo/config.toml`
* Configure `rust-toolchain.toml`
* Add `ra4-hal` to your dependencies
* Call [`ra4_hal::init`](fn@init) from your entry function

Check out the [examples](https://git.sr.ht/~az1/ra4m1-rs/tree/master/item/ra4-examples) and the [Embassy](https://embassy.dev) docs if you're not already familiar.

## Development

The current MSRV is 1.91.
Older versions of Rust may work but this is not guaranteed.

### Rebuilding the metadata bits and bobs

This crate extracts information about peripherals, interrupts, and pin configuration from the FSP pack provided by Renesas.
To update the extracted information:

* Ensure you have a Ruby version manager e.g. [`rvm`](https://rvm.io/) already installed and configured.  The tooling is written in Ruby and tested against v4.
* Download the FSP pack collection e.g. `FSP_Packs_v6.3.1.zip` from <https://github.com/renesas/fsp/releases/>
* Extract the `RA4M1` pack e.g. `Renesas.RA_mcu_ra4m1.6.3.1.pack` from the collection
* Run `meta-gen`

From the workspace root, do this the first time you run meta-gen:
```sh,ignore
$ bundle install --gemfile=./tools/meta-gen/Gemfile
```

Then extract the metadata:
```
$ ./tools/meta-gen/meta-gen.rb Renesas.RA_mcu_ra4m1.6.3.1.pack ra4-hal/meta/
```
