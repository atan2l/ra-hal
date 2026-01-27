# `ra4-hal`

This crate implements a variety of Hardware Abstraction Layer (HAL) traits to support the [Renesas RA4M1](https://www.renesas.com/en/products/ra4m1) microcontrollers.
These traits provide both asynchronous and blocking interfaces that enable compatibility with software like [Embassy](https://embassy.dev).
The end goal is to provide a safe, idiomatic Rust interface that surfaces most of the flexibility provided by the `RA4M1`.

## Notes

### Rebuilding the pinmap

* Get FSP packs from Renesas github <https://github.com/renesas/fsp/releases/download/v6.3.1/FSP_Packs_v6.3.1.zip>
* Run `gen-pinmap`:

```
pushd support/pack-extract; ./gen-pinmap.rb ~/arm/fsp-packs/internal/projectgen/ra/packs/Renesas.RA_mcu_ra4m1.6.3.1.pack  > ../pinmap.yaml; popd
```
