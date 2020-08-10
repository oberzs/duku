// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Sampler - struct that provides image access in shader

use ash::vk;
use std::sync::Arc;

use crate::device::Device;
use crate::error::Result;
use crate::image::TextureFilter;
use crate::image::TextureWrap;

pub(crate) struct Sampler {
    handle: vk::Sampler,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct SamplerOptions {
    pub(crate) anisotropy: f32,
    pub(crate) wrap: TextureWrap,
    pub(crate) filter: TextureFilter,
    pub(crate) mipmaps: bool,
}

impl Sampler {
    pub(crate) fn new(device: &Arc<Device>, options: SamplerOptions) -> Result<Self> {
        let max_lod = if options.mipmaps { 16.0 } else { 0.0 };
        let anisotropy = if options.mipmaps {
            options.anisotropy
        } else {
            0.0
        };
        let mipmap_mode = if options.mipmaps {
            vk::SamplerMipmapMode::LINEAR
        } else {
            vk::SamplerMipmapMode::NEAREST
        };

        let info = vk::SamplerCreateInfo::builder()
            .mag_filter(options.filter.flag())
            .min_filter(options.filter.flag())
            .address_mode_u(options.wrap.flag())
            .address_mode_v(options.wrap.flag())
            .address_mode_w(options.wrap.flag())
            .anisotropy_enable(anisotropy != 0.0)
            .max_anisotropy(anisotropy)
            .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE)
            .unnormalized_coordinates(false)
            .compare_enable(true)
            .compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .mipmap_mode(mipmap_mode)
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
            wrap: TextureWrap::Repeat,
            filter: TextureFilter::Linear,
            mipmaps: true,
        }
    }
}
