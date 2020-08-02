// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it macros

#![macro_use]

macro_rules! cslice {
    ($(
        $(#[$attr:meta])*
        $s:literal
    ),* $(,)?) => (unsafe { &[$(
       $(#[$attr])*
       std::ffi::CStr::from_bytes_with_nul_unchecked(
           concat!($s, "\0").as_bytes()
       ),
    )*]});
}

macro_rules! error {
    ($($arg:expr),*) => {{
        #[cfg(debug_assertions)]
        {
            print!("\x1b[91merror\x1b[0m: ");
            println!($($arg),*);
            std::process::exit(1);
        }
        #[cfg(not(debug_assertions))]
        {
            $(let _ = &$arg;)*
            std::process::exit(1);
        }
    }};
}

macro_rules! warn {
    ($($arg:expr),*) => {{
        #[cfg(debug_assertions)]
        {
            print!("\x1b[93mwarning\x1b[0m: ");
            println!($($arg),*);
        }
        #[cfg(not(debug_assertions))]
        {
            $(let _ = &$arg;)*
        }
    }};
}

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
