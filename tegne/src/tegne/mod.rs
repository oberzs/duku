mod extensions;
mod instance;
mod validator;

use log::debug;
use log::info;

use extensions::Extensions;
use instance::Instance;
use validator::Validator;

pub struct Tegne {
    _validator: Option<Validator>,
    _instance: Instance,
}

impl Tegne {
    pub fn new() -> Self {
        let extensions = Extensions::new();

        debug!("create Vulkan instance");
        let instance = Instance::new(&extensions);
        info!("Vulkan instance created");

        #[cfg(debug_assertions)]
        debug!("create validator");
        #[cfg(debug_assertions)]
        let validator = Some(Validator::new(&instance));
        #[cfg(debug_assertions)]
        info!("validator created");
        #[cfg(not(debug_assertions))]
        let validator = None;

        Self {
            _validator: validator,
            _instance: instance,
        }
    }
}
