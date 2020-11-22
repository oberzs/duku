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

// asserts floats with delta
#[cfg(test)]
macro_rules! assert_eq_delta {
    ($a:expr, $b:expr) => {{
        let eps = 1.0e-6;
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left !== right)` \
        (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
}
