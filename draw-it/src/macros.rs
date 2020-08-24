// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

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

// handles Vulkan errors
macro_rules! vk_check {
    ($e:expr) => {
        match $e {
            vk::SUCCESS => (),
            vk::ERROR_OUT_OF_HOST_MEMORY => panic!("out of host memory"),
            vk::ERROR_OUT_OF_DEVICE_MEMORY => panic!("out of device memory"),
            vk::ERROR_LAYER_NOT_PRESENT => panic!("layer not present"),
            vk::ERROR_EXTENSION_NOT_PRESENT => panic!("extension not present"),
            vk::ERROR_FEATURE_NOT_PRESENT => panic!("feature not present"),
            _ => panic!("Vulkan error {}", $e),
        }
    };
}
