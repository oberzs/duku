fn main() {
    println!(
        "cargo:rustc-link-search='C:/Program Files/mingw/mingw64/lib/gcc/x86_64-w64-mingw32/8.1.0'"
    );
    println!("cargo:rustc-link-lib=libstdc++");
}
