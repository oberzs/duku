use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::version_major;
use ash::vk::version_minor;
use ash::vk::version_patch;
use ash::vk::CommandBuffer;
use ash::vk::DeviceCreateInfo;
use ash::vk::DeviceQueueCreateInfo;
use ash::vk::Extent2D;
use ash::vk::Fence;
use ash::vk::MemoryPropertyFlags;
use ash::vk::PhysicalDevice;
use ash::vk::PhysicalDeviceFeatures;
use ash::vk::PhysicalDeviceMemoryProperties;
use ash::vk::PhysicalDeviceProperties;
use ash::vk::PhysicalDeviceType;
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
use log::info;
use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::ffi::CStr;
use std::sync::Arc;
use tegne_math::clamp;

use super::Commands;
use super::Extensions;
use super::Surface;
use super::Swapchain;
use super::Vulkan;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::sync::fence;
use crate::sync::semaphore;

const IN_FLIGHT_FRAME_COUNT: u32 = 2;

pub(crate) struct Device {
    logical: LogicalDevice,
    _physical: PhysicalDevice,
    properties: DeviceProperties,
    graphics_queue: Queue,
    present_queue: Queue,
    sync_acquire_image: Vec<Semaphore>,
    sync_release_image: Vec<Semaphore>,
    sync_queue_submit: Vec<Fence>,
    commands: RefCell<Vec<Commands>>,
    current_frame: Cell<u32>,
}

pub(crate) struct DeviceProperties {
    pub(crate) _properties: PhysicalDeviceProperties,
    pub(crate) features: PhysicalDeviceFeatures,
    pub(crate) surface_formats: Vec<SurfaceFormatKHR>,
    pub(crate) surface_present_modes: Vec<PresentModeKHR>,
    pub(crate) surface_capabilities: SurfaceCapabilitiesKHR,
    pub(crate) memory_properties: PhysicalDeviceMemoryProperties,
    pub(crate) graphics_index: u32,
    pub(crate) present_index: u32,
    pub(crate) samples: Samples,
    pub(crate) extent: Extent2D,
    pub(crate) present_mode: PresentModeKHR,
    pub(crate) image_count: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Samples(pub(crate) u8);

impl Device {
    pub(crate) fn new(
        vulkan: &Vulkan,
        surface: &Surface,
        exts: &Extensions,
        vsync: bool,
        msaa: u8,
    ) -> Result<Arc<Self>> {
        info!("looking for suitable GPU");

        let gpus = unsafe { vulkan.instance_ref().enumerate_physical_devices()? };

        for physical in gpus.into_iter() {
            let instance = vulkan.instance_ref();

            let (g_index, p_index) = get_queue_indices(vulkan, physical, surface)?;
            let has_queue_indices = g_index.is_some() && p_index.is_some();
            let graphics_index = g_index.unwrap_or_default();
            let present_index = p_index.unwrap_or_default();

            let device_props = unsafe { instance.get_physical_device_properties(physical) };
            let device_features = unsafe { instance.get_physical_device_features(physical) };
            let mem_props = unsafe { instance.get_physical_device_memory_properties(physical) };
            let surface_capabilities = surface.gpu_capabilities(physical)?;

            let samples = pick_samples(device_props, msaa)?;
            let extent = pick_extent(surface_capabilities, surface.width(), surface.height());
            let present_mode = pick_present_mode(vsync);
            let image_count = pick_image_count(surface_capabilities);

            let props = DeviceProperties {
                _properties: device_props,
                features: device_features,
                memory_properties: mem_props,
                surface_formats: surface.gpu_formats(physical)?,
                surface_capabilities,
                surface_present_modes: surface.gpu_present_modes(physical)?,
                graphics_index,
                present_index,
                samples,
                extent,
                present_mode,
                image_count,
            };

            if exts.supports_device(vulkan, physical).is_ok()
                && is_gpu_suitable(&props)
                && has_queue_indices
            {
                let device_name = unsafe { CStr::from_ptr(device_props.device_name.as_ptr()) };
                let device_type = match device_props.device_type {
                    PhysicalDeviceType::DISCRETE_GPU => "(discrete)",
                    PhysicalDeviceType::INTEGRATED_GPU => "(integrated)",
                    PhysicalDeviceType::VIRTUAL_GPU => "(virtual)",
                    _ => "",
                };
                let driver_major = version_major(device_props.driver_version);
                let driver_minor = version_minor(device_props.driver_version);
                let driver_patch = version_patch(device_props.driver_version);

                info!("opening GPU");
                info!("using {:?} {}", device_name, device_type);
                info!(
                    "using driver version {}.{}.{}",
                    driver_major, driver_minor, driver_patch
                );
                info!("using VSync {}", if vsync { "enabled" } else { "disabled" });
                info!("using MSAA level {}", msaa);

                let logical = open_device(physical, vulkan, &props, exts)?;
                let graphics_queue = unsafe { logical.get_device_queue(graphics_index, 0) };
                let present_queue = unsafe { logical.get_device_queue(present_index, 0) };

                let mut sync_acquire_image = vec![];
                let mut sync_release_image = vec![];
                let mut sync_queue_submit = vec![];
                for _ in 0..IN_FLIGHT_FRAME_COUNT {
                    sync_acquire_image.push(semaphore::create(&logical)?);
                    sync_release_image.push(semaphore::create(&logical)?);
                    sync_queue_submit.push(fence::create(&logical)?);
                }

                let device = Arc::new(Self {
                    logical,
                    _physical: physical,
                    properties: props,
                    graphics_queue,
                    present_queue,
                    sync_acquire_image,
                    sync_release_image,
                    sync_queue_submit,
                    commands: RefCell::new(vec![]),
                    current_frame: Cell::new(0),
                });

                for _ in 0..IN_FLIGHT_FRAME_COUNT {
                    device.commands.borrow_mut().push(Commands::new(&device)?);
                }

                return Ok(device);
            }
        }

        Err(ErrorKind::NoSuitableGpu.into())
    }

    pub(crate) fn next_frame(&self, swapchain: &Swapchain) -> Result<()> {
        self.current_frame
            .set((self.current_frame.get() + 1) % IN_FLIGHT_FRAME_COUNT);
        let current = self.current_frame.get() as usize;

        swapchain.next(self.sync_acquire_image[current])?;

        // wait for queue
        let wait = self.sync_queue_submit[current];
        fence::wait_for(&self.logical, wait)?;
        fence::reset(&self.logical, wait)?;

        // reset command recorder
        let cmd = &mut self.commands.borrow_mut()[current];
        cmd.reset()?;
        cmd.begin()?;
        Ok(())
    }

    pub(crate) fn commands(&self) -> Ref<'_, Commands> {
        Ref::map(self.commands.borrow(), |rs| {
            &rs[self.current_frame.get() as usize]
        })
    }

    pub(crate) fn submit_buffer(&self, buffer: CommandBuffer) -> Result<()> {
        let buffers = [buffer];
        let info = SubmitInfo::builder().command_buffers(&buffers).build();
        let infos = [info];

        unsafe {
            self.logical
                .queue_submit(self.graphics_queue, &infos, Fence::null())?;
            self.logical.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn submit(&self) -> Result<()> {
        let current = self.current_frame.get() as usize;
        let wait = [self.sync_acquire_image[current]];
        let signal = [self.sync_release_image[current]];
        let done = self.sync_queue_submit[current];
        let buffers = [self.commands.borrow()[current].end()?];
        let stage_mask = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let info = [SubmitInfo::builder()
            .wait_semaphores(&wait)
            .signal_semaphores(&signal)
            .wait_dst_stage_mask(&stage_mask)
            .command_buffers(&buffers)
            .build()];
        unsafe {
            self.logical
                .queue_submit(self.graphics_queue, &info, done)?
        };
        Ok(())
    }

    pub(crate) fn present(&self, swapchain: &Swapchain) -> Result<()> {
        let current = self.current_frame.get() as usize;
        let wait = self.sync_release_image[current];

        swapchain.present(self.present_queue, wait)?;
        Ok(())
    }

    pub(crate) fn wait_for_idle(&self) -> Result<()> {
        for fen in self.sync_queue_submit.iter() {
            fence::wait_for(&self.logical, *fen)?;
        }

        unsafe {
            self.logical.queue_wait_idle(self.graphics_queue)?;
            self.logical.queue_wait_idle(self.present_queue)?;
            self.logical.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn pick_memory_type(
        &self,
        type_filter: u32,
        props: MemoryPropertyFlags,
    ) -> Result<u32> {
        let mem_type = self
            .properties
            .memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, mem_type)| {
                let byte: u32 = 1 << i;
                (type_filter & byte != 0) && (mem_type.property_flags & props) == props
            })
            .ok_or(ErrorKind::NoSuitableMemoryType)?;
        Ok(mem_type.0 as u32)
    }

    pub(crate) fn is_msaa(&self) -> bool {
        self.properties.samples != Samples(1)
    }

    pub(crate) fn logical(&self) -> &LogicalDevice {
        &self.logical
    }

    pub(crate) fn properties(&self) -> &DeviceProperties {
        &self.properties
    }

    pub(crate) fn are_indices_unique(&self) -> bool {
        self.properties.graphics_index != self.properties.present_index
    }

    pub(crate) fn indices(&self) -> Vec<u32> {
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
            self.commands
                .borrow_mut()
                .iter_mut()
                .for_each(|r| r.manual_drop(&self.logical));
            self.logical.destroy_device(None);
        }
    }
}

impl Samples {
    pub(crate) fn flag(&self) -> Result<SampleCountFlags> {
        let flag = match self.0 {
            1 => SampleCountFlags::TYPE_1,
            2 => SampleCountFlags::TYPE_2,
            4 => SampleCountFlags::TYPE_4,
            8 => SampleCountFlags::TYPE_8,
            16 => SampleCountFlags::TYPE_16,
            32 => SampleCountFlags::TYPE_32,
            64 => SampleCountFlags::TYPE_64,
            _ => return Err(ErrorKind::InvalidMsaa.into()),
        };
        Ok(flag)
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
    surface: &Surface,
) -> Result<(Option<u32>, Option<u32>)> {
    let mut graphics = None;
    let mut present = None;

    let queue_properties = unsafe {
        vulkan
            .instance_ref()
            .get_physical_device_queue_family_properties(device)
    };
    for (i, props) in queue_properties.iter().enumerate() {
        let present_support = surface.supports_device(device, i as u32)?;
        let graphics_support = props.queue_flags.contains(QueueFlags::GRAPHICS);

        if props.queue_count > 0 && present_support {
            present = Some(i as u32);
        }
        if props.queue_count > 0 && graphics_support {
            graphics = Some(i as u32);
        }
    }
    Ok((graphics, present))
}

fn open_device(
    physical: PhysicalDevice,
    vulkan: &Vulkan,
    props: &DeviceProperties,
    exts: &Extensions,
) -> Result<LogicalDevice> {
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

    let logical = unsafe { vulkan.instance_ref().create_device(physical, &info, None)? };
    Ok(logical)
}

fn pick_samples(properties: PhysicalDeviceProperties, msaa: u8) -> Result<Samples> {
    let counts = properties.limits.framebuffer_color_sample_counts
        & properties.limits.framebuffer_depth_sample_counts;

    let samples = Samples(msaa);

    if !counts.contains(samples.flag()?) {
        Err(ErrorKind::UnsupportedMsaa.into())
    } else {
        Ok(samples)
    }
}

fn pick_extent(capabilities: SurfaceCapabilitiesKHR, width: u32, height: u32) -> Extent2D {
    let extent = capabilities.current_extent;
    let min_width = capabilities.min_image_extent.width;
    let max_width = capabilities.max_image_extent.width;
    let min_height = capabilities.min_image_extent.height;
    let max_height = capabilities.max_image_extent.height;

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

fn pick_present_mode(vsync: bool) -> PresentModeKHR {
    if vsync {
        PresentModeKHR::FIFO
    } else {
        PresentModeKHR::IMMEDIATE
    }
}

fn pick_image_count(capabilities: SurfaceCapabilitiesKHR) -> u32 {
    let min_image_count = capabilities.min_image_count;
    let max_image_count = capabilities.max_image_count;
    if max_image_count > 0 && min_image_count + 1 > max_image_count {
        max_image_count
    } else {
        min_image_count + 1
    }
}
