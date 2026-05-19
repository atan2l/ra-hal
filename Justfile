mod doc 'just/doc.just'
mod examples

[private]
default:
  @just --list --list-submodules

# Runs clippy for a given chip.
[arg('chip', pattern='(ra[02468][adelmpt][1-9])')]
clippy chip *args:
    @just clippy-{{ chip }} {{ args }}

[private]
clippy-ra2a1 *args:
    cargo clippy --target thumbv8m.base-none-eabi --no-deps --package ra-hal --features _doc_ra2a1,defmt {{ args }}

[private]
clippy-ra4l1 *args:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra4l1,defmt {{ args }}

[private]
clippy-ra4m1 *args:
    cargo clippy --target thumbv7em-none-eabihf --features _doc_ra4m1,defmt {{ args }}

[private]
clippy-ra6m5 *args:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra6m5,defmt {{ args}}

[private]
clippy-ra8m1 *args:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features _doc_ra8m1,defmt {{ args }}

[private]
publish-crate *args:
    cargo publish --package ra-hal --features _doc_ra4m1 --target thumbv7em-none-eabihf {{ args }}
