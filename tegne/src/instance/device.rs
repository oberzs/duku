use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::ColorSpaceKHR;
use ash::vk::CommandBuffer;
use ash::vk::DeviceCreateInfo;
use ash::vk::DeviceQueueCreateInfo;
use ash::vk::Extent2D;
use ash::vk::Fence;
use ash::vk::Format;
use ash::vk::MemoryPropertyFlags;
use ash::vk::PhysicalDevice;
use ash::vk::PhysicalDeviceFeatures;
use ash::vk::PhysicalDeviceMemoryProperties;
use ash::vk::PhysicalDeviceProperties;
use ash::vk::PipelineStageFlags;
use ash::vk::PresentModeKHR;
use ash::vk::Queue;
use ash::vk::QueueFlags;
use ash::vk::SampleCountFlags;
use ash::vk::Semaphore;
use ash::vk::SubmitInfo;
use ash::vk::SurfaceCapabilitiesKHR;
use ash::vk::SurfaceFormatKHR;
use ash::Device as LogicalDevice;
use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use super::CommandRecorder;
use super::Extensions;
use super::Swapchain;
use super::Vulkan;
use super::WindowSurface;
use crate::sync::fence;
use crate::sync::semaphore;
use crate::utils::clamp;
use crate::utils::error;
use crate::utils::OrError;

const IN_FLIGHT_FRAME_COUNT: u32 = 2;

pub struct Device {
    logical: LogicalDevice,
    _physical: PhysicalDevice,
    properties: DeviceProperties,
    graphics_queue: Queue,
    _present_queue: Queue,
    sync_acquire_image: Vec<Semaphore>,
    sync_release_image: Vec<Semaphore>,
    sync_queue_submit: Vec<Fence>,
    command_recorders: RefCell<Vec<CommandRecorder>>,
    current_frame: Cell<u32>,
}

pub struct DeviceProperties {
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
        vulkan: &Vulkan,
        surface: &WindowSurface,
        exts: &Extensions,
        vsync: VSync,
        msaa: u8,
    ) -> Rc<Self> {
        let gpus = unsafe {
            vulkan
                .instance_ref()
                .enumerate_physical_devices()
                .or_error("cannot find a GPU")
        };

        for physical in gpus.into_iter() {
            let instance = vulkan.instance_ref();

            let (g_index, p_index) = get_queue_indices(vulkan, physical, surface);
            let has_queue_indices = g_index.is_some() && p_index.is_some();
            let graphics_index = g_index.unwrap_or_default();
            let present_index = p_index.unwrap_or_default();

            let device_props = unsafe { instance.get_physical_device_properties(physical) };
            let device_features = unsafe { instance.get_physical_device_features(physical) };
            let mem_props = unsafe { instance.get_physical_device_memory_properties(physical) };

            let props = DeviceProperties {
                properties: device_props,
                features: device_features,
                memory_properties: mem_props,
                surface_formats: surface.gpu_formats(physical),
                surface_capabilities: surface.gpu_capabilities(physical),
                surface_present_modes: surface.gpu_present_modes(physical),
                graphics_index,
                present_index,
                msaa,
                vsync,
            };

            if exts.supports_device(vulkan, physical)
                && is_gpu_suitable(&props)
                && has_queue_indices
            {
                let logical = open_device(physical, vulkan, &props, exts);
                let graphics_queue = unsafe { logical.get_device_queue(graphics_index, 0) };
                let present_queue = unsafe { logical.get_device_queue(present_index, 0) };

                let sync_acquire_image = (0..IN_FLIGHT_FRAME_COUNT)
                    .map(|_| semaphore::create(&logical))
                    .collect::<Vec<_>>();
                let sync_release_image = (0..IN_FLIGHT_FRAME_COUNT)
                    .map(|_| semaphore::create(&logical))
                    .collect::<Vec<_>>();
                let sync_queue_submit = (0..IN_FLIGHT_FRAME_COUNT)
                    .map(|_| fence::create(&logical))
                    .collect::<Vec<_>>();

                let device = Rc::new(Self {
                    logical,
                    _physical: physical,
                    properties: props,
                    graphics_queue,
                    _present_queue: present_queue,
                    sync_acquire_image,
                    sync_release_image,
                    sync_queue_submit,
                    command_recorders: RefCell::new(vec![]),
                    current_frame: Cell::new(0),
                });

                *device.command_recorders.borrow_mut() = (0..IN_FLIGHT_FRAME_COUNT)
                    .map(|_| CommandRecorder::new(&device))
                    .collect::<Vec<_>>();

                return device;
            }
        }

        error("cannot find suitable GPU");
    }

    pub fn next_frame(&self) {
        self.current_frame
            .set((self.current_frame.get() + 1) % IN_FLIGHT_FRAME_COUNT);
        let current = self.current_frame.get() as usize;

        // wait for queue
        let wait = self.sync_queue_submit[current];
        fence::wait_for(&self.logical, wait);
        fence::reset(&self.logical, wait);

        // reset command recorder
        let recorder = &mut self.command_recorders.borrow_mut()[current];
        recorder.reset();
        recorder.begin();
    }

    pub fn record_commands(&self) -> Ref<'_, CommandRecorder> {
        Ref::map(self.command_recorders.borrow(), |rs| {
            &rs[self.current_frame.get() as usize]
        })
    }

    pub fn submit_buffer(&self, buffer: CommandBuffer) {
        let buffers = [buffer];
        let info = SubmitInfo::builder().command_buffers(&buffers).build();
        let infos = [info];

        unsafe {
            self.logical
                .queue_submit(self.graphics_queue, &infos, Fence::null())
                .or_error("cannot submit command buffer");
            self.logical
                .device_wait_idle()
                .or_error("cannot wait device idle");
        }
    }

    pub fn submit(&self) {
        let current = self.current_frame.get() as usize;
        let wait = [self.sync_acquire_image[current]];
        let signal = [self.sync_release_image[current]];
        let done = self.sync_queue_submit[current];
        let buffers = [self.command_recorders.borrow()[current].end()];
        let stage_mask = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let info = [SubmitInfo::builder()
            .wait_semaphores(&wait)
            .signal_semaphores(&signal)
            .wait_dst_stage_mask(&stage_mask)
            .command_buffers(&buffers)
            .build()];
        unsafe {
            self.logical
                .queue_submit(self.graphics_queue, &info, done)
                .or_error("cannot submit draw command buffer")
        };
    }

    pub fn present(&self, swapchain: &Swapchain) {
        let current = self.current_frame.get() as usize;
        let wait = self.sync_release_image[current];

        swapchain.present(self.graphics_queue, wait);
    }

    pub fn pick_memory_type(&self, type_filter: u32, props: MemoryPropertyFlags) -> u32 {
        self.properties
            .memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, mem_type)| {
                (type_filter & (1 << i) as u32 != 0) && (mem_type.property_flags & props) == props
            })
            .or_error("cannot find suitable memory type")
            .0 as u32
    }

    pub fn pick_sample_count(&self) -> SampleCountFlags {
        let counts = self
            .properties
            .properties
            .limits
            .framebuffer_color_sample_counts
            & self
                .properties
                .properties
                .limits
                .framebuffer_depth_sample_counts;

        let count = match self.properties.msaa {
            1 => SampleCountFlags::TYPE_1,
            2 => SampleCountFlags::TYPE_2,
            4 => SampleCountFlags::TYPE_4,
            8 => SampleCountFlags::TYPE_8,
            16 => SampleCountFlags::TYPE_16,
            32 => SampleCountFlags::TYPE_32,
            64 => SampleCountFlags::TYPE_64,
            _ => error("invalid msaa value"),
        };

        if !counts.contains(count) {
            error("unsupported msaa value");
        }

        count
    }

    pub fn pick_extent(&self, width: u32, height: u32) -> Extent2D {
        let extent = self.properties.surface_capabilities.current_extent;
        let min_width = self.properties.surface_capabilities.min_image_extent.width;
        let max_width = self.properties.surface_capabilities.max_image_extent.width;
        let min_height = self.properties.surface_capabilities.min_image_extent.height;
        let max_height = self.properties.surface_capabilities.max_image_extent.height;

        if extent.width != u32::max_value() {
            extent
        } else {
            let w = clamp(width, min_width, max_width);
            let h = clamp(height, min_height, max_height);
            Extent2D {
                width: w,
                height: h,
            }
        }
    }

    pub fn pick_present_mode(&self) -> PresentModeKHR {
        match self.properties.vsync {
            VSync::Enabled => PresentModeKHR::FIFO,
            VSync::Disabled => PresentModeKHR::IMMEDIATE,
        }
    }

    pub fn pick_image_count(&self) -> u32 {
        let min_image_count = self.properties.surface_capabilities.min_image_count;
        let max_image_count = self.properties.surface_capabilities.max_image_count;
        if max_image_count > 0 && min_image_count + 1 > max_image_count {
            max_image_count
        } else {
            min_image_count + 1
        }
    }

    pub fn pick_depth_format(&self) -> Format {
        Format::D32_SFLOAT_S8_UINT
    }

    pub fn pick_rgba_format(&self) -> Format {
        Format::R8G8B8A8_UNORM
    }

    pub fn pick_bgra_format(&self) -> Format {
        Format::B8G8R8A8_UNORM
    }

    pub fn pick_color_space(&self) -> ColorSpaceKHR {
        ColorSpaceKHR::SRGB_NONLINEAR
    }

    pub fn is_msaa(&self) -> bool {
        self.pick_sample_count() != SampleCountFlags::TYPE_1
    }

    pub fn logical(&self) -> &LogicalDevice {
        &self.logical
    }

    pub fn properties(&self) -> &DeviceProperties {
        &self.properties
    }

    pub fn are_indices_unique(&self) -> bool {
        self.properties.graphics_index != self.properties.present_index
    }

    pub fn indices(&self) -> Vec<u32> {
        vec![
            self.properties.graphics_index,
            self.properties.present_index,
        ]
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.sync_acquire_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.logical, *s));
            self.sync_release_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.logical, *s));
            self.sync_queue_submit
                .iter()
                .for_each(|f| fence::destroy(&self.logical, *f));
            self.logical.destroy_device(None);
        }
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
    vulkan: &Vulkan,
    device: PhysicalDevice,
    surface: &WindowSurface,
) -> (Option<u32>, Option<u32>) {
    let mut graphics = None;
    let mut present = None;

    let queue_properties = unsafe {
        vulkan
            .instance_ref()
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
    vulkan: &Vulkan,
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
        vulkan
            .instance_ref()
            .create_device(physical, &info, None)
            .or_error("cannot open GPU")
    }
}
