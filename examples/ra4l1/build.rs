fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");

    println!("cargo:rustc-link-arg-bins=-Tra-link.x");
    println!("cargo:rustc-link-arg-bins=-Tra-device.x");

    #[cfg(feature = "defmt")]
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    Ok(())
}
