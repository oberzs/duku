// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Links in Vulkan Loader

use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/vulkan", manifest_dir);
    println!("cargo:rustc-link-lib=vulkan");
}
