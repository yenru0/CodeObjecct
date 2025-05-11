use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").unwrap();
    let exe_name = env::var("CARGO_PKG_NAME").unwrap();

    println!("cargo::warning={}", profile);

    let target_path = Path::new(&target_dir).join(&profile).join(&exe_name);
    let dest_path = Path::new("../../build").join("rs.out");

    if target_path.exists() {
        fs::copy(&target_path, &dest_path).expect("Failed to copy executable");
    }
}
