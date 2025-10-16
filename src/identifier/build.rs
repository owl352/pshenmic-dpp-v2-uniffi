use std::path::PathBuf;
use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let udl_path = PathBuf::from(manifest_dir)
        .join("src")
        .join("identifier.udl");

    // Convert to string and pass to uniffi_build
    let udl_path_str = udl_path.to_str().expect("UDL path is not valid UTF-8");
    uniffi::generate_scaffolding(udl_path_str).unwrap_or_else(|err| {
        panic!("{err}")
    });
}