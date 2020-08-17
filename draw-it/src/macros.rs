// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it macros

#![macro_use]

// creates a slice of CStr, used for extensions
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

// handles Vulkan errors
// macro_rules! vk_check {}
