// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// tegne-import macros

#![macro_use]

macro_rules! error {
    ($($arg:expr),*) => {{
        print!("\x1b[91merror\x1b[0m: ");
        println!($($arg),*);
        std::process::exit(1);
    }};
}

macro_rules! warn {
    ($($arg:expr),*) => {{
        print!("\x1b[93mwarning\x1b[0m: ");
        println!($($arg),*);
    }};
}
