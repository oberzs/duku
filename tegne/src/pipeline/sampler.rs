// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Sampler - struct that provides image access in shader

use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

use super::SamplerAddress;
use super::SamplerFilter;
use crate::error::Result;
use crate::device::Device;

pub(crate) struct Sampler {
    handle: vk::Sampler,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct SamplerOptions {
    pub(crate) anisotropy: f32,
    pub(crate) address: SamplerAddress,
    pub(crate) filter: SamplerFilter,
}

impl Sampler {
    pub(crate) fn new(device: &Arc<Device>, options: SamplerOptions) -> Result<Self> {
        let info = vk::SamplerCreateInfo::builder()
            .mag_filter(options.filter.flag())
            .min_filter(options.filter.flag())
            .address_mode_u(options.address.flag())
            .address_mode_v(options.address.flag())
            .address_mode_w(options.address.flag())
            .anisotropy_enable(options.anisotropy != 0.0)
            .max_anisotropy(options.anisotropy)
            .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(16.0);

        let handle = unsafe { device.logical().create_sampler(&info, None)? };

        Ok(Self {
            handle,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn handle(&self) -> vk::Sampler {
        self.handle
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_sampler(self.handle, None);
        }
    }
}

impl Default for SamplerOptions {
    fn default() -> Self {
        Self {
            anisotropy: 0.0,
            address: SamplerAddress::Repeat,
            filter: SamplerFilter::Linear,
        }
    }
}
