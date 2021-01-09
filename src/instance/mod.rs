// Oliver Berzs
// https://github.com/oberzs/duku

mod properties;
mod version;

use std::ffi::CStr;
use std::mem;
use std::ptr;

use crate::surface::Surface;
use crate::vk;

pub(crate) use properties::GpuProperties;
pub(crate) use properties::SurfaceProperties;
pub(crate) use version::Version;

pub(crate) const DEVICE_EXTENSIONS: &[&str] = &["VK_KHR_swapchain"];
const INSTANCE_EXTENSIONS: &[&str] = &[
    "VK_KHR_surface",
    #[cfg(target_os = "windows")]
    "VK_KHR_win32_surface",
    #[cfg(target_os = "linux")]
    "VK_KHR_xlib_surface",
    #[cfg(target_os = "macos")]
    "VK_EXT_metal_surface",
];

pub(crate) struct Instance {
    handle: vk::Instance,
    gpus: Vec<vk::PhysicalDevice>,
}

impl Instance {
    pub(crate) fn new() -> Self {
        info!("initializing the Vulkan API");

        // log version information
        let mut vk_version = 0;
        unsafe {
            vk::check(vk::enumerate_instance_version(&mut vk_version));
        }
        let version = Version::from_vk(vk_version);
        info!("using Vulkan {}", version);

        // enumerate extensions
        let extensions = unsafe {
            let mut count = 0;
            vk::check(vk::enumerate_instance_extension_properties(
                ptr::null(),
                &mut count,
                ptr::null_mut(),
            ));
            let mut properties: Vec<vk::ExtensionProperties> = Vec::with_capacity(count as usize);
            vk::check(vk::enumerate_instance_extension_properties(
                ptr::null(),
                &mut count,
                properties.as_mut_ptr(),
            ));
            properties.set_len(count as usize);
            properties
        };

        // pick extensions
        let mut picked_extensions = vec![];
        for ext in &extensions {
            let name = unsafe {
                CStr::from_ptr(ext.extension_name.as_ptr())
                    .to_str()
                    .expect("CStr is not valid UTF-8")
            };
            if INSTANCE_EXTENSIONS.contains(&name) {
                picked_extensions.push(ext.extension_name.as_ptr());
            }
        }
        for ext in INSTANCE_EXTENSIONS {
            info!("using extension '{}'", ext);
        }

        // create instance
        let app_info = vk::ApplicationInfo {
            s_type: vk::STRUCTURE_TYPE_APPLICATION_INFO,
            p_application_name: ptr::null(),
            application_version: 0,
            p_engine_name: ptr::null(),
            engine_version: 0,
            api_version: vk_version,
            p_next: ptr::null(),
        };
        let instance_info = vk::InstanceCreateInfo {
            s_type: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            enabled_extension_count: picked_extensions.len() as u32,
            pp_enabled_extension_names: picked_extensions.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            flags: 0,
            p_next: ptr::null(),
        };
        let mut handle = 0;
        unsafe {
            vk::check(vk::create_instance(
                &instance_info,
                ptr::null(),
                &mut handle,
            ));
        }

        // enumerate GPUs on system
        let gpus = unsafe {
            let mut count = 0;
            vk::check(vk::enumerate_physical_devices(
                handle,
                &mut count,
                ptr::null_mut(),
            ));
            let mut devices: Vec<vk::PhysicalDevice> = Vec::with_capacity(count as usize);
            vk::check(vk::enumerate_physical_devices(
                handle,
                &mut count,
                devices.as_mut_ptr(),
            ));
            devices.set_len(count as usize);
            devices
        };

        Self { handle, gpus }
    }

    pub(crate) fn gpu_properties(&self) -> Vec<GpuProperties> {
        self.gpus
            .iter()
            .map(|gpu| {
                // enumerate extensions
                let extensions = unsafe {
                    let mut count = 0;
                    vk::check(vk::enumerate_device_extension_properties(
                        *gpu,
                        ptr::null(),
                        &mut count,
                        ptr::null_mut(),
                    ));
                    let mut properties: Vec<vk::ExtensionProperties> =
                        Vec::with_capacity(count as usize);
                    vk::check(vk::enumerate_device_extension_properties(
                        *gpu,
                        ptr::null(),
                        &mut count,
                        properties.as_mut_ptr(),
                    ));
                    properties.set_len(count as usize);
                    properties
                };

                // check extension support
                let supports_extensions = extensions.iter().any(|e| {
                    let name = unsafe {
                        CStr::from_ptr(e.extension_name.as_ptr())
                            .to_str()
                            .expect("CStr is not valid UTF-8")
                    };
                    !INSTANCE_EXTENSIONS.contains(&name)
                });

                // get device things
                let mut properties = unsafe { mem::zeroed() };
                let mut features = unsafe { mem::zeroed() };
                let mut memory = unsafe { mem::zeroed() };
                unsafe {
                    vk::get_physical_device_properties(*gpu, &mut properties);
                    vk::get_physical_device_features(*gpu, &mut features);
                    vk::get_physical_device_memory_properties(*gpu, &mut memory);
                }

                // get queue index
                let mut queue_index = None;

                let families = queue_families(*gpu);

                for (i, props) in families.iter().enumerate() {
                    let graphics_support = (props.queue_flags & vk::QUEUE_GRAPHICS_BIT) != 0;

                    if props.queue_count > 0 && graphics_support {
                        queue_index = Some(i as u32);
                    }
                }

                // add gpu properties
                GpuProperties {
                    properties,
                    features,
                    memory,
                    queue_index,
                    supports_extensions,
                }
            })
            .collect()
    }

    pub(crate) fn surface_properties(&self, surface: &Surface) -> Vec<SurfaceProperties> {
        self.gpus
            .iter()
            .map(|gpu| {
                // get surface properties
                let mut capabilities = unsafe { mem::zeroed() };
                unsafe {
                    vk::check(vk::get_physical_device_surface_capabilities_khr(
                        *gpu,
                        surface.handle(),
                        &mut capabilities,
                    ));
                }
                let formats = unsafe {
                    let mut count = 0;
                    vk::check(vk::get_physical_device_surface_formats_khr(
                        *gpu,
                        surface.handle(),
                        &mut count,
                        ptr::null_mut(),
                    ));
                    let mut fs: Vec<vk::SurfaceFormatKHR> = Vec::with_capacity(count as usize);
                    vk::check(vk::get_physical_device_surface_formats_khr(
                        *gpu,
                        surface.handle(),
                        &mut count,
                        fs.as_mut_ptr(),
                    ));
                    fs.set_len(count as usize);
                    fs
                };
                let present_modes = unsafe {
                    let mut count = 0;
                    vk::check(vk::get_physical_device_surface_present_modes_khr(
                        *gpu,
                        surface.handle(),
                        &mut count,
                        ptr::null_mut(),
                    ));
                    let mut pms: Vec<vk::PresentModeKHR> = Vec::with_capacity(count as usize);
                    vk::check(vk::get_physical_device_surface_present_modes_khr(
                        *gpu,
                        surface.handle(),
                        &mut count,
                        pms.as_mut_ptr(),
                    ));
                    pms.set_len(count as usize);
                    pms
                };

                // get queue index
                let mut queue_index = None;

                let families = queue_families(*gpu);

                for (i, props) in families.iter().enumerate() {
                    let graphics_support = (props.queue_flags & vk::QUEUE_GRAPHICS_BIT) != 0;

                    let present_support = unsafe {
                        let mut support = 0;
                        vk::check(vk::get_physical_device_surface_support_khr(
                            *gpu,
                            i as u32,
                            surface.handle(),
                            &mut support,
                        ));
                        support != vk::FALSE
                    };

                    if props.queue_count > 0 && graphics_support && present_support {
                        queue_index = Some(i as u32);
                    }
                }

                // pick extent
                let extent = capabilities.current_extent;

                // pick image count
                let min_image_count = capabilities.min_image_count;
                let max_image_count = capabilities.max_image_count;
                let image_count = if max_image_count > 0 && min_image_count + 1 > max_image_count {
                    max_image_count
                } else {
                    min_image_count + 1
                };

                // add surface properties
                SurfaceProperties {
                    formats,
                    present_modes,
                    capabilities,
                    extent,
                    image_count,
                    queue_index,
                }
            })
            .collect()
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn create_win32_surface(
        &self,
        info: &vk::Win32SurfaceCreateInfoKHR,
    ) -> vk::SurfaceKHR {
        let mut surface = 0;
        unsafe {
            vk::check(vk::create_win32_surface_khr(
                self.handle,
                info,
                ptr::null(),
                &mut surface,
            ));
        }
        surface
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn create_linux_surface(
        &self,
        info: &vk::XlibSurfaceCreateInfoKHR,
    ) -> vk::SurfaceKHR {
        let mut surface = 0;
        unsafe {
            vk::check(vk::create_xlib_surface_khr(
                self.handle,
                info,
                ptr::null(),
                &mut surface,
            ));
        }
        surface
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn create_macos_surface(
        &self,
        info: &vk::MacOSSurfaceCreateInfoMVK,
    ) -> vk::SurfaceKHR {
        let mut surface = 0;
        unsafe {
            vk::check(vk::create_mac_ossurface_mvk(
                self.handle,
                info,
                ptr::null(),
                &mut surface,
            ));
        }
        surface
    }

    pub(crate) fn destroy_surface(&self, surface: &Surface) {
        unsafe {
            vk::destroy_surface_khr(self.handle, surface.handle(), ptr::null());
        }
    }

    pub(crate) fn create_device(
        &self,
        gpu_index: usize,
        info: &vk::DeviceCreateInfo,
    ) -> vk::Device {
        let mut device = 0;
        unsafe {
            vk::check(vk::create_device(
                self.gpus[gpu_index],
                info,
                ptr::null(),
                &mut device,
            ));
        }
        device
    }

    pub(crate) fn destroy(&self) {
        unsafe { vk::destroy_instance(self.handle, ptr::null()) };
    }
}

fn queue_families(gpu: vk::PhysicalDevice) -> Vec<vk::QueueFamilyProperties> {
    unsafe {
        let mut count = 0;
        vk::get_physical_device_queue_family_properties(gpu, &mut count, ptr::null_mut());
        let mut fams: Vec<vk::QueueFamilyProperties> = Vec::with_capacity(count as usize);
        vk::get_physical_device_queue_family_properties(gpu, &mut count, fams.as_mut_ptr());
        fams.set_len(count as usize);
        fams
    }
}
