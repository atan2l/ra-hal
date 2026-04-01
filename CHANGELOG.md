# Changelog for `ra-hal`

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project will eventually adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once the 1.0 milestone is reached.

## [0.2.4] - TBD
### Security
- RUSTSEC-2026-0009: Updated `vergen-gitcl`
### Added
- dmac
  * pac: New transforms
  * Transfer API
- dtc: read (peripheral-to-memory) function
- gpio: `embedded-hal` input and output traits
- pwm
  * `embedded-hal` Pwm trait
  * Quadrature decoding
- spi:
  * pac: Add SPI byte access register
  * API for DMAC an DTC backed transfers
  * Example to read from a Bosch BMI-160 sensor with the `mini-sensors` library
### Changed
- embassy: Update to latest published versions
- icu: Mark `icu_enable` as unsafe for the same reasons `IRQ.enable` is unsafe
- pac:
  * Change some SPI enum names around
  * Add symbolic variants to GPT debounce fields
  * gpt: Rename reserved enum variants
- spi:
  * Use more appropriate language for SPI pin names
  * Implement `SpiBus` for blocking and async (DMAC, DTC)
  * Allow specifying arbitrary bit rates
  * Updated examples

## [0.2.3] - 2026-03-09
### Added
- iwdt: new driver
- wdt: new driver
- pac: `OSM` peripheral with `OFS0` and `OFS1` registers
- mstp: `ModuleStop` trait to provide a consistent interface to clock gating
- gpt: Add 32-bit `GPT` support.
### Changed
- time-driver: Make generic over 32-bit `GPT` instances.
### Fixed
- gpt: Ensure `embassy-time` uses the correct tick rate when ƒHOCO=64 MHz
- system: Enable high speed mode for ƒICLK=32 MHz

## [0.2.2] - 2026-03-02
### Added
- dac
  * Support for basic configuration and output configuration
  * Example showing DAC+DTC usage
- dtc: DTC transfer API
- elc
  * pac: New transforms
  * hal: API for triggering software generated interrupts
- fcache: Add off-by-default feature to enable the flash cache
- gpt: General purpose timer API (16-bit only)
### Changed
- crc: `feed_bytes` no longer returns computed value
- elc: Rename `ElcSwevt0` and `ElcSwevt1`
- Trait names harmonized across different drivers

## [0.2.1] - 2026-02-20
### Added
- gpio
  * Support for open-drain and pull-up manipulation on pins that support these features.
  * Sanity checks on `PinId` constructors.
- pwm
  * Initial vestiges of error handling in `set_frequency`.
- spi: new driver
- uart
  * Support for `ƒHOCO` = 32, 64 MHz.
### Changed
- gpio
  * Optimize pin state manipulators.
  * Hide pins that can be used for `SWD` behind `swd-as-gpio` feature.
- osm
  * Moved configuration to `ra4-hal` library.
- pwm
  * `Config` is now `non-exhaustive`.
  * Rationalize struct names.
- uart
  * Rationalize struct names.
