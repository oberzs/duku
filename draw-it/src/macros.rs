// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it macros

#![macro_use]

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

macro_rules! profile_scope {
    ($name:expr) => {
        #[cfg(feature = "profiler")]
        let _f =
            crate::profiler::ProfileTimer::new(concat!(module_path!(), "::", $name, ":", line!()));
    };
}
