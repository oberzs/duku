// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// picks the most suitable GPU

use ash::vk;
use std::ffi::CStr;

use crate::error::ErrorKind;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::instance::GPUProperties;
use crate::surface::ColorSpace;
use crate::surface::VSync;

pub(crate) fn pick_gpu(
    gpu_properties: &[GPUProperties],
    vsync: VSync,
    msaa: Msaa,
) -> Result<usize> {
    info!("looking for suitable GPU");

    // score each GPU based on properties
    let mut scores = gpu_properties
        .iter()
        .enumerate()
        .map(|(i, props)| {
            let mut score = 1;

            // optional
            if props.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                score += 100;
            }

            // mandatory
            if !props.supports_extensions {
                score = 0;
            }
            if props.graphics_index.is_none() || props.present_index.is_none() {
                score = 0;
            }
            if props.features.sampler_anisotropy == 0 {
                score = 0;
            }
            if props.features.fill_mode_non_solid == 0 {
                score = 0;
            }
            if props.features.wide_lines == 0 {
                score = 0;
            }
            if !props.supports_present_mode(vsync) {
                score = 0;
            }
            if !props.supports_msaa(msaa) {
                score = 0;
            }
            if !props.formats.contains(&vk::SurfaceFormatKHR {
                color_space: ColorSpace::Srgb.flag(),
                format: ImageFormat::Sbgra.flag(),
            }) {
                score = 0;
            }

            (i, score)
        })
        .collect::<Vec<_>>();

    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores.retain(|s| s.1 > 0);

    match scores.first() {
        None => Err(ErrorKind::NoSuitableGpu.into()),
        Some((picked, _)) => {
            // log picked GPU information
            let info = gpu_properties[*picked].properties;
            let device_name = unsafe { CStr::from_ptr(info.device_name.as_ptr()) };
            let device_type = match info.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => "(discrete)",
                vk::PhysicalDeviceType::INTEGRATED_GPU => "(integrated)",
                vk::PhysicalDeviceType::VIRTUAL_GPU => "(virtual)",
                _ => "",
            };
            let driver_major = vk::version_major(info.driver_version);
            let driver_minor = vk::version_minor(info.driver_version);
            let driver_patch = vk::version_patch(info.driver_version);

            info!("using {:?} {}", device_name, device_type);
            info!(
                "using driver version {}.{}.{}",
                driver_major, driver_minor, driver_patch
            );

            Ok(*picked)
        }
    }
}
