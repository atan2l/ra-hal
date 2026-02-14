use std::{env, fs::File, io::Write, path::PathBuf};

fn add_linker_script(in_filename: &str, out_filename: &str) {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let input_data = std::fs::read(in_filename)
        .unwrap_or_else(|e| panic!("Couldn't read: {in_filename}\n{e:#?}"));

    let out_path = out_dir.join(out_filename);

    File::create(&out_path)
        .unwrap()
        .write_all(&input_data)
        .unwrap_or_else(|e| panic!("Couldn't write: {out_path:?}\n{e:#?}"));

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed={}", in_filename);
}

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        add_linker_script("device.x", "ra-device.x");
    }
    add_linker_script("ra-link.x", "ra-link.x");

    println!("cargo:rerun-if-changed=build.rs");
}
