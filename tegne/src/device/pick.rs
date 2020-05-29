// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// picks the most suitable GPU

use ash::vk;
use log::info;
use std::ffi::CStr;

use super::DeviceProperties;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::profile_scope;
use crate::surface::SurfaceProperties;

pub(crate) fn pick_gpu(
    s_props: &[SurfaceProperties],
    d_props: &[DeviceProperties],
) -> Result<usize> {
    profile_scope!("pick_gpu");
    info!("looking for suitable GPU");

    for (i, (s, d)) in s_props.iter().zip(d_props.iter()).enumerate() {
        let supports_extensions = d.supports_extensions;

        let has_queue_indices = s.graphics_index.is_some() && s.present_index.is_some();
        let has_surface_formats = !s.formats.is_empty();
        let has_surface_present_modes = !s.present_modes.is_empty();

        let has_sampler_anisotropy = d.features.sampler_anisotropy > 0;
        let has_line_mode = d.features.fill_mode_non_solid > 0;
        let has_wide_lines = d.features.wide_lines > 0;

        if supports_extensions
            && has_queue_indices
            && has_surface_formats
            && has_surface_present_modes
            && has_sampler_anisotropy
            && has_line_mode
            && has_wide_lines
        {
            // log picked GPU information
            let device_name = unsafe { CStr::from_ptr(d.properties.device_name.as_ptr()) };
            let device_type = match d.properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => "(discrete)",
                vk::PhysicalDeviceType::INTEGRATED_GPU => "(integrated)",
                vk::PhysicalDeviceType::VIRTUAL_GPU => "(virtual)",
                _ => "",
            };
            let driver_major = vk::version_major(d.properties.driver_version);
            let driver_minor = vk::version_minor(d.properties.driver_version);
            let driver_patch = vk::version_patch(d.properties.driver_version);

            info!("found GPU");
            info!("using {:?} {}", device_name, device_type);
            info!(
                "using driver version {}.{}.{}",
                driver_major, driver_minor, driver_patch
            );

            return Ok(i);
        }
    }

    Err(ErrorKind::NoSuitableGpu.into())
}
