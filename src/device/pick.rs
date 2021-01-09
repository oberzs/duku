// Oliver Berzs
// https://github.com/oberzs/duku

use std::ffi::CStr;

use crate::image::Format;
use crate::image::Msaa;
use crate::instance::GpuProperties;
use crate::instance::SurfaceProperties;
use crate::instance::Version;
use crate::surface::VSync;
use crate::vk;

pub(crate) fn pick_gpu(
    gpu_properties: &[GpuProperties],
    surface_properties: &Option<Vec<SurfaceProperties>>,
    vsync: VSync,
    msaa: Msaa,
) -> usize {
    info!("looking for suitable GPU");

    // score each GPU based on properties
    let mut scores: Vec<_> = gpu_properties
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let mut score = 1;

            // optional
            if g.properties.device_type == vk::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {
                score += 100;
            }

            // mandatory
            if !g.supports_extensions {
                score = 0;
            }
            if g.queue_index.is_none() {
                score = 0;
            }
            if g.features.sampler_anisotropy == 0 {
                score = 0;
            }
            if g.features.fill_mode_non_solid == 0 {
                score = 0;
            }
            if g.features.wide_lines == 0 {
                score = 0;
            }
            if !g.supports_msaa(msaa) {
                score = 0;
            }

            if let Some(s) = surface_properties {
                if !s[i].supports_present_mode(vsync) {
                    score = 0;
                }
                if s[i].queue_index.is_none() {
                    score = 0;
                }
                if s[i].capabilities.current_extent.width == u32::max_value() {
                    // happens for Wayland window surfaces
                    score = 0;
                }
                let format = s[i].formats.iter().find(|f| {
                    f.color_space == vk::COLOR_SPACE_SRGB_NONLINEAR_KHR
                        && f.format == Format::Bgra.flag()
                });
                if format.is_none() {
                    score = 0;
                }
            }

            (i, score)
        })
        .collect();

    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores.retain(|s| s.1 > 0);

    let (picked, _) = scores.first().expect("no suitable GPU");

    // log picked GPU information
    let info = &gpu_properties[*picked].properties;
    let device_name = unsafe { CStr::from_ptr(info.device_name.as_ptr()) };
    let device_type = match info.device_type {
        vk::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => "(discrete)",
        vk::PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => "(integrated)",
        vk::PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => "(virtual)",
        _ => "",
    };
    let version = Version::from_vk(info.driver_version);

    info!("using {:?} {}", device_name, device_type);
    info!("using driver version {}", version);

    *picked
}
