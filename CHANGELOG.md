# Changelog for `ra4-hal` and `ra4m1-ctpac`

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project will eventually adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once the 1.0 milestone is reached.

## [0.2.3] - TBD
### Added
- iwdt: new driver
- wdt: new driver
- pac: `OSM` peripheral with `OFS0` and `OFS1` registers

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
