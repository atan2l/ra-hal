use std::{env, fs::File, io::Write, path::PathBuf};

fn add_linker_script(input_path: &str, output_path: &str) {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let input_data = std::fs::read(input_path).unwrap();

    File::create(out.join(output_path))
        .unwrap()
        .write_all(&input_data)
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", input_path);
}

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        add_linker_script("device.x", "ra-device.x");
    }
    add_linker_script("../support/ra-link.x", "ra-link.x");

    println!("cargo:rerun-if-changed=build.rs");
}
