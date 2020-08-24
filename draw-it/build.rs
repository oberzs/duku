// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Links in Vulkan Loader

use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    println!(
        "cargo:rustc-link-search={}/vulkan",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=vulkan");
}
