mod examples

[private]
default:
  @just --list --list-submodules

# Runs clippy for a given chip.
[arg('chip', pattern='(ra[02468][adelmpt][1-9])')]
clippy chip:
    @just clippy-{{ chip }}

[private]
clippy-ra4l1:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features ra4l1,hoco_80mhz,mem_ram64k_flash512k,100lqfp,defmt

[private]
clippy-ra4m1:
    cargo clippy --target thumbv7em-none-eabihf --features ra4m1,hoco_48mhz,64lqfp,defmt

[private]
clippy-ra6m5:
    cargo clippy --target thumbv8m.main-none-eabihf --no-deps --package ra-hal --features ra6m5,hoco_20mhz,mem_ram512k_flash2048k,176lqfp,defmt

# Generates documentation for the `RA2A1`.
[arg('flag', pattern='^$|--open')]
doc-ra2a1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.base-none-eabi --features _doc_ra2a1 --target-dir target/doc-ra2a1 {{ flag }}

# Generates documentation for the `RA4L1`.
[arg('flag', pattern='^$|--open')]
doc-ra4l1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.main-none-eabihf --features _doc_ra4l1 --target-dir target/doc-ra4l1 {{ flag }}

# Generates documentation for the `RA4M1`.
[arg('flag', pattern='^$|--open')]
doc-ra4m1 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv7em-none-eabihf --features _doc_ra4m1 --target-dir target/doc-ra4m1 {{ flag }}

# Generates documentation for the `RA6M5`.
[arg('flag', pattern='^$|--open')]
doc-ra6m5 flag="":
  cargo doc --no-deps --package ra-hal --target thumbv8m.main-none-eabihf --features _doc_ra6m5 --target-dir target/doc-ra6m5 {{ flag }}

[private]
doc-ra4m1-publish:
  cargo doc --no-deps --package ra-hal --target thumbv7em-none-eabihf --features _doc_ra4m1 --target-dir target/doc-ra4m1
  tar -C target/doc-ra4m1/thumbv7em-none-eabihf/doc/ -czvf doc-ra4m1.tar.gz .
  hut pages publish -d az1.srht.site -s ra4m1 doc-ra4m1.tar.gz
  rm doc-ra4m1.tar.gz
