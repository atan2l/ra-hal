mod doc 'just/doc.just'
mod examples

[private]
default:
  @just --list --list-submodules

# Runs clippy for a given chip.
[arg('chip', pattern='(ra[02468][adelmpt][1-9])')]
clippy chip:
    @just clippy-{{ chip }}

[private]
clippy-ra2a1:
    cargo clippy --target thumbv8m.base-none-eabi --no-deps --package ra-hal --features _doc_ra2a1,defmt

[private]
clippy-ra4l1:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra4l1,defmt

[private]
clippy-ra4m1:
    cargo clippy --target thumbv7em-none-eabihf --features _doc_ra4m1,defmt

[private]
clippy-ra6m5:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra6m5,defmt

[private]
clippy-ra8m1:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra8m1,defmt
