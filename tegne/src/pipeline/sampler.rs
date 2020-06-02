// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Sampler - struct that provides image access in shader

use ash::vk;
use std::sync::Arc;

use super::SamplerAddress;
use super::SamplerFilter;
use super::SamplerMipmaps;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct Sampler {
    handle: vk::Sampler,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct SamplerOptions {
    pub(crate) anisotropy: f32,
    pub(crate) address: SamplerAddress,
    pub(crate) filter: SamplerFilter,
    pub(crate) mipmaps: SamplerMipmaps,
}

impl Sampler {
    pub(crate) fn new(device: &Arc<Device>, options: SamplerOptions) -> Result<Self> {
        let max_lod = match options.mipmaps {
            SamplerMipmaps::Enabled => 16.0,
            SamplerMipmaps::Disabled => 0.0,
        };

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
            .mipmap_mode(options.mipmaps.flag())
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(max_lod);

        let handle = device.create_sampler(&info)?;

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
        self.device.destroy_sampler(self.handle);
    }
}

impl Default for SamplerOptions {
    fn default() -> Self {
        Self {
            anisotropy: 0.0,
            address: SamplerAddress::Repeat,
            filter: SamplerFilter::Linear,
            mipmaps: SamplerMipmaps::Enabled,
        }
    }
}
