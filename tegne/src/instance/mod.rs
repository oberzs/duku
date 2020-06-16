// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Instance - struct for Vulkan entrypoint

mod extension;
mod layer;

#[cfg(debug_assertions)]
mod validator;

use ash::extensions::khr::Surface as SurfaceExt;
use ash::extensions::khr::Swapchain as SwapchainExt;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::Device as VkDevice;
use ash::Entry;
use ash::Instance as VkInstance;
use std::ffi::CStr;
use std::ffi::CString;

#[cfg(debug_assertions)]
use ash::extensions::ext::DebugUtils as DebugExt;

use crate::error::ErrorKind;
use crate::error::Result;
use crate::surface::Surface;

pub(crate) struct Instance {
    handle: VkInstance,
    entry: Entry,
    gpus: Vec<vk::PhysicalDevice>,
    surface_ext: SurfaceExt,

    #[cfg(debug_assertions)]
    messenger: vk::DebugUtilsMessengerEXT,
    #[cfg(debug_assertions)]
    debug_ext: DebugExt,
}

impl Instance {
    pub(crate) fn new() -> Result<Self> {
        profile_scope!("new");
        info!("initializing the Vulkan API");

        let entry = Entry::new()?;

        // log version information
        match entry.try_enumerate_instance_version()? {
            Some(version) => {
                let major = vk::version_major(version);
                let minor = vk::version_minor(version);
                let patch = vk::version_patch(version);
                info!("using Vulkan {}.{}.{}", major, minor, patch);
            }
            None => info!("using Vulkan 1.0"),
        }

        // check extension support
        let extension_list = extension::list();
        let available_extensions = entry
            .enumerate_instance_extension_properties()?
            .iter()
            .map(|e| {
                let ptr = e.extension_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();
        for extension in extension_list.iter() {
            if !available_extensions.contains(&extension) {
                return Err(ErrorKind::UnsupportedExtension(extension.clone()).into());
            }
        }

        // check validation layer support
        let layer_list = layer::list();
        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| {
                let ptr = l.layer_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();
        for layer in layer_list.iter() {
            if !available_layers.contains(&layer) {
                return Err(ErrorKind::UnsupportedValidation(layer.clone()).into());
            }
        }

        // create instance
        let extensions = extension::to_i8(&extension_list);
        let layers = layer::to_i8(&layer_list);
        let app_info = vk::ApplicationInfo::builder().api_version(vk::make_version(1, 2, 0));

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        let handle = unsafe { entry.create_instance(&info, None)? };

        // create validator if in debug mode
        #[cfg(debug_assertions)]
        let (debug_ext, messenger) = {
            let debug_ext = DebugExt::new(&entry, &handle);
            let config = validator::messenger_config();
            let messenger = unsafe { debug_ext.create_debug_utils_messenger(&config, None)? };
            (debug_ext, messenger)
        };

        // create extensions
        let surface_ext = SurfaceExt::new(&entry, &handle);

        // enumerate GPUs on system
        let gpus = unsafe { handle.enumerate_physical_devices()? };

        Ok(Self {
            handle,
            entry,
            gpus,
            surface_ext,

            #[cfg(debug_assertions)]
            messenger,
            #[cfg(debug_assertions)]
            debug_ext,
        })
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn create_surface(
        &self,
        info: &vk::Win32SurfaceCreateInfoKHR,
    ) -> Result<vk::SurfaceKHR> {
        use ash::extensions::khr::Win32Surface;
        let loader = Win32Surface::new(&self.entry, &self.handle);
        Ok(unsafe { loader.create_win32_surface(&info, None)? })
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn create_surface(
        &self,
        info: &vk::XlibSurfaceCreateInfoKHR,
    ) -> Result<vk::SurfaceKHR> {
        use ash::extensions::khr::XlibSurface;
        let loader = XlibSurface::new(&self.entry, &self.handle);
        Ok(unsafe { loader.create_xlib_surface(&info, None)? })
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn create_surface(
        &self,
        info: &vk::MacOSSurfaceCreateInfoMVK,
    ) -> Result<vk::SurfaceKHR> {
        use ash::extensions::mvk::MacOSSurface;
        let loader = MacOSSurface::new(&self.entry, &self.handle);
        Ok(unsafe { loader.create_mac_os_surface_mvk(&info, None)? })
    }

    pub(crate) fn destroy_surface(&self, handle: vk::SurfaceKHR) {
        unsafe {
            self.surface_ext.destroy_surface(handle, None);
        }
    }

    pub(crate) fn get_surface_formats(
        &self,
        surface: &Surface,
    ) -> Result<Vec<Vec<vk::SurfaceFormatKHR>>> {
        let mut formats = vec![];
        for gpu in self.gpus.iter() {
            let fs = unsafe {
                self.surface_ext
                    .get_physical_device_surface_formats(*gpu, surface.handle())?
            };
            formats.push(fs);
        }
        Ok(formats)
    }

    pub(crate) fn get_surface_present_modes(
        &self,
        surface: &Surface,
    ) -> Result<Vec<Vec<vk::PresentModeKHR>>> {
        let mut modes = vec![];
        for gpu in self.gpus.iter() {
            let pms = unsafe {
                self.surface_ext
                    .get_physical_device_surface_present_modes(*gpu, surface.handle())?
            };
            modes.push(pms);
        }
        Ok(modes)
    }

    pub(crate) fn get_surface_capabilities(
        &self,
        surface: &Surface,
    ) -> Result<Vec<vk::SurfaceCapabilitiesKHR>> {
        let mut caps = vec![];
        for gpu in self.gpus.iter() {
            let cs = unsafe {
                self.surface_ext
                    .get_physical_device_surface_capabilities(*gpu, surface.handle())?
            };
            caps.push(cs);
        }
        Ok(caps)
    }

    pub(crate) fn get_surface_queue_indices(
        &self,
        surface: &Surface,
    ) -> Result<Vec<(Option<u32>, Option<u32>)>> {
        let mut indices = vec![];
        for gpu in self.gpus.iter() {
            let mut graphics = None;
            let mut present = None;

            let properties = unsafe {
                self.handle
                    .get_physical_device_queue_family_properties(*gpu)
            };

            for (i, props) in properties.iter().enumerate() {
                let present_support = self.get_surface_support(surface, *gpu, i)?;
                let graphics_support = props.queue_flags.contains(vk::QueueFlags::GRAPHICS);

                if props.queue_count > 0 && present_support {
                    present = Some(i as u32);
                }
                if props.queue_count > 0 && graphics_support {
                    graphics = Some(i as u32);
                }
            }

            indices.push((graphics, present));
        }
        Ok(indices)
    }

    pub(crate) fn get_device_properties(&self) -> Vec<vk::PhysicalDeviceProperties> {
        self.gpus
            .iter()
            .map(|gpu| unsafe { self.handle.get_physical_device_properties(*gpu) })
            .collect()
    }

    pub(crate) fn get_device_features(&self) -> Vec<vk::PhysicalDeviceFeatures> {
        self.gpus
            .iter()
            .map(|gpu| unsafe { self.handle.get_physical_device_features(*gpu) })
            .collect()
    }

    pub(crate) fn get_device_memory(&self) -> Vec<vk::PhysicalDeviceMemoryProperties> {
        self.gpus
            .iter()
            .map(|gpu| unsafe { self.handle.get_physical_device_memory_properties(*gpu) })
            .collect()
    }

    pub(crate) fn get_device_extensions(&self) -> Result<Vec<Vec<CString>>> {
        let mut exts = vec![];
        for gpu in self.gpus.iter() {
            let es = unsafe {
                self.handle
                    .enumerate_device_extension_properties(*gpu)?
                    .iter()
                    .map(|e| {
                        let ptr = e.extension_name.as_ptr();
                        CStr::from_ptr(ptr).to_owned()
                    })
                    .collect::<Vec<_>>()
            };
            exts.push(es);
        }
        Ok(exts)
    }

    pub(crate) fn create_device(
        &self,
        gpu_index: usize,
        info: &vk::DeviceCreateInfo,
    ) -> Result<VkDevice> {
        Ok(unsafe {
            self.handle
                .create_device(self.gpus[gpu_index], info, None)?
        })
    }

    pub(crate) fn create_swapchain_extension(&self, device: &VkDevice) -> SwapchainExt {
        SwapchainExt::new(&self.handle, device)
    }

    fn get_surface_support(
        &self,
        surface: &Surface,
        gpu: vk::PhysicalDevice,
        queue_index: usize,
    ) -> Result<bool> {
        Ok(unsafe {
            self.surface_ext.get_physical_device_surface_support(
                gpu,
                queue_index as u32,
                surface.handle(),
            )?
        })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            #[cfg(debug_assertions)]
            self.debug_ext
                .destroy_debug_utils_messenger(self.messenger, None);
            self.handle.destroy_instance(None);
        };
    }
}
