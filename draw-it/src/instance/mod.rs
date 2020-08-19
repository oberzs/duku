// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Instance - struct for Vulkan entrypoint

mod extension;
mod properties;

use ash::extensions::khr::Surface as SurfaceExt;
use ash::extensions::khr::Swapchain as SwapchainExt;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::Device as VkDevice;
use ash::Entry;
use ash::Instance as VkInstance;
use std::cmp;
use std::ffi::CStr;

use crate::error::Result;
use crate::surface::Surface;
use extension::INSTANCE_EXTENSIONS;

pub(crate) use extension::DEVICE_EXTENSIONS;
pub(crate) use properties::GPUProperties;

pub(crate) struct Instance {
    handle: VkInstance,
    entry: Entry,
    gpus: Vec<vk::PhysicalDevice>,
    surface_ext: SurfaceExt,
}

impl Instance {
    pub(crate) fn new() -> Result<Self> {
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
        let available_extensions = entry
            .enumerate_instance_extension_properties()?
            .iter()
            .map(|e| {
                let ptr = e.extension_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();
        INSTANCE_EXTENSIONS.assert_missing(&available_extensions)?;

        // create instance
        let extensions = INSTANCE_EXTENSIONS.as_ptr();
        let app_info = vk::ApplicationInfo::builder().api_version(vk::make_version(1, 2, 0));

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions);

        let handle = unsafe { entry.create_instance(&info, None)? };

        // create extensions
        let surface_ext = SurfaceExt::new(&entry, &handle);

        // enumerate GPUs on system
        let gpus = unsafe { handle.enumerate_physical_devices()? };

        Ok(Self {
            surface_ext,
            handle,
            entry,
            gpus,
        })
    }

    pub(crate) fn gpu_properties(&self, surface: &Surface) -> Result<Vec<GPUProperties>> {
        let mut gpu_properties = vec![];
        for gpu in &self.gpus {
            unsafe {
                // check extension support
                let available_extensions = self
                    .handle
                    .enumerate_device_extension_properties(*gpu)?
                    .iter()
                    .map(|e| {
                        let ptr = e.extension_name.as_ptr();
                        CStr::from_ptr(ptr).to_owned()
                    })
                    .collect::<Vec<_>>();
                let supports_extensions = DEVICE_EXTENSIONS
                    .assert_missing(&available_extensions)
                    .is_ok();

                // get device things
                let properties = self.handle.get_physical_device_properties(*gpu);
                let features = self.handle.get_physical_device_features(*gpu);
                let memory = self.handle.get_physical_device_memory_properties(*gpu);

                // get surface things
                let formats = self
                    .surface_ext
                    .get_physical_device_surface_formats(*gpu, surface.handle())?;
                let present_modes = self
                    .surface_ext
                    .get_physical_device_surface_present_modes(*gpu, surface.handle())?;
                let capabilities = self
                    .surface_ext
                    .get_physical_device_surface_capabilities(*gpu, surface.handle())?;

                // get queue index
                let mut queue_index = None;

                let families = self
                    .handle
                    .get_physical_device_queue_family_properties(*gpu);

                for (i, props) in families.iter().enumerate() {
                    let present_support = self.surface_ext.get_physical_device_surface_support(
                        *gpu,
                        i as u32,
                        surface.handle(),
                    )?;
                    let graphics_support = props.queue_flags.contains(vk::QueueFlags::GRAPHICS);

                    if props.queue_count > 0 && present_support && graphics_support {
                        queue_index = Some(i as u32);
                    }
                }

                // pick extent
                let mut extent = capabilities.current_extent;
                let min_width = capabilities.min_image_extent.width;
                let max_width = capabilities.max_image_extent.width;
                let min_height = capabilities.min_image_extent.height;
                let max_height = capabilities.max_image_extent.height;

                if extent.width == u32::max_value() {
                    let width = cmp::max(cmp::min(surface.width(), max_width), min_width);
                    let height = cmp::max(cmp::min(surface.height(), max_height), min_height);
                    extent = vk::Extent2D { width, height };
                }

                // pick image count
                let min_image_count = capabilities.min_image_count;
                let max_image_count = capabilities.max_image_count;
                let image_count = if max_image_count > 0 && min_image_count + 1 > max_image_count {
                    max_image_count
                } else {
                    min_image_count + 1
                };

                // add gpu properties
                gpu_properties.push(GPUProperties {
                    properties,
                    features,
                    memory,
                    capabilities,
                    formats,
                    present_modes,
                    queue_index,
                    extent,
                    image_count,
                    supports_extensions,
                });
            }
        }
        Ok(gpu_properties)
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
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_instance(None);
        };
    }
}
