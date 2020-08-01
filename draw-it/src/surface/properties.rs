// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// SurfaceProperties - properties for the specific surface

use ash::vk;

use super::Surface;
use crate::error::Result;
use crate::instance::Instance;
use crate::math::clamp;

pub(crate) struct SurfaceProperties {
    pub(crate) formats: Vec<vk::SurfaceFormatKHR>,
    pub(crate) capabilities: vk::SurfaceCapabilitiesKHR,
    pub(crate) graphics_index: Option<u32>,
    pub(crate) present_index: Option<u32>,
    pub(crate) extent: vk::Extent2D,
    pub(crate) present_mode: PresentMode,
    pub(crate) image_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColorSpace {
    Srgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PresentMode {
    Fifo,
    Immediate,
    Disabled,
}

impl SurfaceProperties {
    pub(crate) fn new(instance: &Instance, surface: &Surface, vsync: bool) -> Result<Vec<Self>> {
        let formats = instance.get_surface_formats(surface)?.into_iter();
        let present_modes = instance.get_surface_present_modes(surface)?.into_iter();
        let capabilities = instance.get_surface_capabilities(surface)?.into_iter();
        let queue_indices = instance.get_surface_queue_indices(surface)?.into_iter();

        Ok(formats
            .zip(present_modes.zip(capabilities.zip(queue_indices)))
            .map(|(f, (p, (c, q)))| {
                let present_mode = pick_present_mode(&p, vsync);
                let extent = pick_extent(c, surface);
                let image_count = pick_image_count(c);

                Self {
                    formats: f,
                    capabilities: c,
                    graphics_index: q.0,
                    present_index: q.1,
                    extent,
                    present_mode,
                    image_count,
                }
            })
            .collect())
    }

    pub(crate) fn refresh(
        &mut self,
        instance: &Instance,
        surface: &Surface,
        gpu_index: usize,
    ) -> Result<()> {
        // refresh surface properties when window changes state
        self.formats = instance.get_surface_formats(surface)?.remove(gpu_index);
        self.capabilities = instance
            .get_surface_capabilities(surface)?
            .remove(gpu_index);
        self.extent = pick_extent(self.capabilities, surface);
        Ok(())
    }

    pub(crate) fn graphics_index(&self) -> u32 {
        self.graphics_index.expect("no graphics index")
    }

    pub(crate) fn present_index(&self) -> u32 {
        self.present_index.expect("no present index")
    }

    pub(crate) fn are_indices_unique(&self) -> bool {
        self.graphics_index() != self.present_index()
    }

    pub(crate) fn indices(&self) -> [u32; 2] {
        [self.graphics_index(), self.present_index()]
    }
}

impl ColorSpace {
    pub(crate) fn flag(&self) -> vk::ColorSpaceKHR {
        vk::ColorSpaceKHR::SRGB_NONLINEAR
    }
}

impl PresentMode {
    pub(crate) fn flag(&self) -> vk::PresentModeKHR {
        match *self {
            Self::Fifo => vk::PresentModeKHR::FIFO,
            Self::Immediate => vk::PresentModeKHR::IMMEDIATE,
            Self::Disabled => unreachable!(),
        }
    }
}

fn pick_extent(capabilities: vk::SurfaceCapabilitiesKHR, surface: &Surface) -> vk::Extent2D {
    let extent = capabilities.current_extent;
    let min_width = capabilities.min_image_extent.width;
    let max_width = capabilities.max_image_extent.width;
    let min_height = capabilities.min_image_extent.height;
    let max_height = capabilities.max_image_extent.height;

    if extent.width != u32::max_value() {
        extent
    } else {
        let width = clamp(surface.width(), min_width, max_width);
        let height = clamp(surface.height(), min_height, max_height);
        vk::Extent2D { width, height }
    }
}

fn pick_present_mode(present_modes: &[vk::PresentModeKHR], vsync: bool) -> PresentMode {
    info!("using VSync {}", if vsync { "enabled" } else { "disabled" });
    let mut present_mode = if vsync {
        PresentMode::Fifo
    } else {
        PresentMode::Immediate
    };

    if !present_modes.contains(&present_mode.flag()) {
        present_mode = PresentMode::Disabled;
    }

    present_mode
}

fn pick_image_count(capabilities: vk::SurfaceCapabilitiesKHR) -> u32 {
    let min_image_count = capabilities.min_image_count;
    let max_image_count = capabilities.max_image_count;
    if max_image_count > 0 && min_image_count + 1 > max_image_count {
        max_image_count
    } else {
        min_image_count + 1
    }
}
