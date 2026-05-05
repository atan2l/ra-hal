set working-directory := '..'

[private]
default:
  @just --list --list-submodules

# Generates documentation for the `RA2A1`.
[arg('flag', pattern='^$|--open')]
ra2a1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.base-none-eabi --features _doc_ra2a1 --target-dir target/doc-ra2a1 {{ flag }}

# Generates documentation for the `RA4L1`.
[arg('flag', pattern='^$|--open')]
ra4l1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.main-none-eabihf --features _doc_ra4l1 --target-dir target/doc-ra4l1 {{ flag }}

# Generates documentation for the `RA4M1`.
[arg('flag', pattern='^$|--open')]
ra4m1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv7em-none-eabihf --features _doc_ra4m1 --target-dir target/doc-ra4m1 {{ flag }}

# Generates documentation for the `RA6M5`.
[arg('flag', pattern='^$|--open')]
ra6m5 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.main-none-eabihf --features _doc_ra6m5 --target-dir target/doc-ra6m5 {{ flag }}

# Generates documentation for the `RA8M1`.
[arg('flag', pattern='^$|--open')]
ra8m1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.main-none-eabihf --features _doc_ra8m1 --target-dir target/doc-ra8m1 {{ flag }}
