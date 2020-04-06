use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::DeviceCreateInfo;
use ash::vk::DeviceQueueCreateInfo;
use ash::vk::PhysicalDevice;
use ash::vk::PhysicalDeviceFeatures;
use ash::vk::PhysicalDeviceMemoryProperties;
use ash::vk::PhysicalDeviceProperties;
use ash::vk::PresentModeKHR;
use ash::vk::Queue;
use ash::vk::QueueFlags;
use ash::vk::SurfaceCapabilitiesKHR;
use ash::vk::SurfaceFormatKHR;
use ash::Device as LogicalDevice;

use super::Extensions;
use super::Instance;
use super::WindowSurface;
use crate::utils::error;
use crate::utils::OrError;

pub struct Device {
    logical: LogicalDevice,
    _physical: PhysicalDevice,
    _properties: DeviceProperties,
    _graphics_queue: Queue,
    _present_queue: Queue,
}

struct DeviceProperties {
    pub properties: PhysicalDeviceProperties,
    pub features: PhysicalDeviceFeatures,
    pub surface_formats: Vec<SurfaceFormatKHR>,
    pub surface_present_modes: Vec<PresentModeKHR>,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
    pub memory_properties: PhysicalDeviceMemoryProperties,
    pub graphics_index: u32,
    pub present_index: u32,
    pub vsync: VSync,
    pub msaa: u8,
}

#[derive(Copy, Clone)]
pub enum VSync {
    Enabled,
    Disabled,
}

impl Device {
    pub fn new(
        instance: &Instance,
        surface: &WindowSurface,
        exts: &Extensions,
        vsync: VSync,
        msaa: u8,
    ) -> Self {
        let devices = unsafe {
            instance
                .vk_ref()
                .enumerate_physical_devices()
                .or_error("cannot find a GPU")
        };

        for device in devices.into_iter() {
            let i = instance.vk_ref();

            let (g_index, p_index) = get_queue_indices(instance, device, surface);
            let has_queue_indices = g_index.is_some() && p_index.is_some();
            let graphics_index = g_index.unwrap_or_default();
            let present_index = p_index.unwrap_or_default();

            let device_props = unsafe { i.get_physical_device_properties(device) };
            let device_features = unsafe { i.get_physical_device_features(device) };
            let mem_props = unsafe { i.get_physical_device_memory_properties(device) };

            let props = DeviceProperties {
                properties: device_props,
                features: device_features,
                memory_properties: mem_props,
                surface_formats: surface.gpu_formats(device),
                surface_capabilities: surface.gpu_capabilities(device),
                surface_present_modes: surface.gpu_present_modes(device),
                graphics_index,
                present_index,
                msaa,
                vsync,
            };

            if exts.supports_device(instance, device)
                && is_gpu_suitable(&props)
                && has_queue_indices
            {
                let logical = open_device(device, instance, &props, exts);
                let graphics_queue = unsafe { logical.get_device_queue(graphics_index, 0) };
                let present_queue = unsafe { logical.get_device_queue(present_index, 0) };

                return Self {
                    logical,
                    _physical: device,
                    _properties: props,
                    _graphics_queue: graphics_queue,
                    _present_queue: present_queue,
                };
            }
        }

        error("cannot find suitable GPU");
    }

    pub fn logical(&self) -> &LogicalDevice {
        &self.logical
    }
}

fn is_gpu_suitable(props: &DeviceProperties) -> bool {
    let surface_support_adequate =
        !props.surface_formats.is_empty() && !props.surface_present_modes.is_empty();

    surface_support_adequate
        && props.features.sampler_anisotropy > 0
        && props.features.fill_mode_non_solid > 0
        && props.features.wide_lines > 0
}

fn get_queue_indices(
    instance: &Instance,
    device: PhysicalDevice,
    surface: &WindowSurface,
) -> (Option<u32>, Option<u32>) {
    let mut graphics = None;
    let mut present = None;

    let queue_properties = unsafe {
        instance
            .vk_ref()
            .get_physical_device_queue_family_properties(device)
    };
    queue_properties.iter().enumerate().for_each(|(i, props)| {
        let present_support = surface.supports_device(device, i as u32);
        let graphics_support = props.queue_flags.contains(QueueFlags::GRAPHICS);

        if props.queue_count > 0 && present_support {
            present = Some(i as u32);
        }
        if props.queue_count > 0 && graphics_support {
            graphics = Some(i as u32);
        }
    });
    (graphics, present)
}

fn open_device(
    physical: PhysicalDevice,
    instance: &Instance,
    props: &DeviceProperties,
    exts: &Extensions,
) -> LogicalDevice {
    let features = PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .fill_mode_non_solid(true)
        .wide_lines(true);

    let g_index = props.graphics_index;
    let p_index = props.present_index;
    let queue_priorities = [1.0];

    let g_queue_info = DeviceQueueCreateInfo::builder()
        .queue_family_index(g_index)
        .queue_priorities(&queue_priorities)
        .build();
    let p_queue_info = DeviceQueueCreateInfo::builder()
        .queue_family_index(p_index)
        .queue_priorities(&queue_priorities)
        .build();

    let mut queue_infos = vec![g_queue_info];
    if g_index != p_index {
        queue_infos.push(p_queue_info);
    }

    let layers = exts.layers();
    let extensions = exts.device();

    let info = DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_features(&features)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions);

    unsafe {
        instance
            .vk_ref()
            .create_device(physical, &info, None)
            .or_error("cannot open GPU")
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.logical.destroy_device(None);
        }
    }
}
