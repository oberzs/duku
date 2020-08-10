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

// Ref helper macro to generate struct action
macro_rules! ref_action {
    ($name:ident) => {
        pub fn $name(&self) {
            self.with(|a| a.$name());
        }
    };
}

// Ref helper macro to generate struct setter
macro_rules! ref_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self, value: $type) {
            self.with(|a| a.$name(value));
        }
    };
}

// Ref helper macro to generate struct setter from Into implementer
macro_rules! ref_into_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self, value: impl Into<$type>) {
            self.with(|a| a.$name(value));
        }
    };
}

// Ref helper macro to generate struct getter
macro_rules! ref_getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            self.with(|a| a.$name.clone())
        }
    };
}
