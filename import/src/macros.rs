// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// tegne-import macros

#![macro_use]

macro_rules! error {
    ($($arg:tt)*) => {{
        print!("{}: ", console::style("error").red().bright());
        println!($($arg)*);
        std::process::exit(1);
    }};
}

macro_rules! warn {
    ($($arg:tt)*) => {{
        print!("{}: ", console::style("warning").yellow());
        println!($($arg)*);
    }};
}
