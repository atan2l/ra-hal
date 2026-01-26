# `ra4-hal`

This crate implements a variety of Hardware Abstraction Layer (HAL) traits to support the [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) microcontrollers.
These traits provide both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev).
The end goal is to provide a safe, idiomatic Rust interface that surfaces most of the flexibility provided by the `RA4M1`.
