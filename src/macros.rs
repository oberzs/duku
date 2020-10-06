// Oliver Berzs
// https://github.com/oberzs/draw-it

// draw-it macros

#![macro_use]

// prints debug info in debug mode
macro_rules! info {
    ($($arg:expr),*) => {{
        #[cfg(debug_assertions)]
        {
            print!("\x1b[96minfo\x1b[0m: ");
            println!($($arg),*);
        }
        #[cfg(not(debug_assertions))]
        {
            $(let _ = &$arg;)*
        }
    }};
}

// prints warning in debug mode
#[cfg(feature = "glsl")]
macro_rules! warn {
    ($($arg:expr),*) => {{
        #[cfg(debug_assertions)]
        {
            print!("\x1b[93mwarn\x1b[0m: ");
            println!($($arg),*);
        }
        #[cfg(not(debug_assertions))]
        {
            $(let _ = &$arg;)*
        }
    }};
}
