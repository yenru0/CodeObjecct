use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=src/main.rs");
    // TODO: change to always execute build.rs
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").unwrap();
    let exe_name = env::var("CARGO_PKG_NAME").unwrap();

    println!("cargo::warning={}", profile);

    let target_path = Path::new(&target_dir).join(&profile).join(&exe_name);
    let dest_path = Path::new("../../build").join("rs.out");
    println!("cargo::warning={}", exe_name);
    if target_path.exists() {
        println!("cargo::warning=copy to {:#?}", dest_path);
        match fs::copy(&target_path, &dest_path) {
            Ok(_) => println!("cargo::warning=copy success"),
            Err(_) => println!("cargo::warning=copy failed"),
        }
    }
}
