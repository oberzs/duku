// Oliver Berzs
// https://github.com/oberzs/duku

#![macro_use]

// prints debug info in debug mode
macro_rules! info {
    ($($arg:expr),*) => {{
        #[cfg(feature = "log")]
        {
            print!("\x1b[96minfo\x1b[0m: ");
            println!($($arg),*);
        }
        #[cfg(not(feature = "log"))]
        {
            $(let _ = &$arg;)*
        }
    }};
}

// prints warning in debug mode
#[cfg(feature = "glsl")]
macro_rules! warn {
    ($($arg:expr),*) => {{
        #[cfg(feature = "log")]
        {
            print!("\x1b[93mwarn\x1b[0m: ");
            println!($($arg),*);
        }
        #[cfg(not(feature = "log"))]
        {
            $(let _ = &$arg;)*
        }
    }};
}
