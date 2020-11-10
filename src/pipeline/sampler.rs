// Oliver Berzs
// https://github.com/oberzs/duku

// Sampler - struct that provides image access in shader

use std::ptr;

use crate::device::Device;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::vk;

pub(crate) struct Sampler {
    handle: vk::Sampler,
}

impl Sampler {
    pub(crate) fn new(
        device: &Device,
        wrap: TextureWrap,
        filter: TextureFilter,
        mipmaps: bool,
        anisotropy: f32,
    ) -> Self {
        let max_lod = if mipmaps { 16.0 } else { 0.0 };
        let mipmap_mode = if mipmaps {
            vk::SAMPLER_MIPMAP_MODE_LINEAR
        } else {
            vk::SAMPLER_MIPMAP_MODE_NEAREST
        };
        let max_anisotropy = if mipmaps { anisotropy } else { 0.0 };
        let anisotropy_enable = if mipmaps { vk::TRUE } else { vk::FALSE };

        let info = vk::SamplerCreateInfo {
            s_type: vk::STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            mag_filter: filter.flag(),
            min_filter: filter.flag(),
            address_mode_u: wrap.flag(),
            address_mode_v: wrap.flag(),
            address_mode_w: wrap.flag(),
            mip_lod_bias: 0.0,
            compare_enable: vk::TRUE,
            compare_op: vk::COMPARE_OP_LESS_OR_EQUAL,
            min_lod: 0.0,
            border_color: vk::BORDER_COLOR_FLOAT_OPAQUE_WHITE,
            unnormalized_coordinates: vk::FALSE,
            mipmap_mode,
            anisotropy_enable,
            max_anisotropy,
            max_lod,
        };

        let handle = device.create_sampler(&info);

        Self { handle }
    }

    pub(crate) fn destroy(&self, device: &Device) {
        device.destroy_sampler(self.handle);
    }

    pub(crate) const fn handle(&self) -> vk::Sampler {
        self.handle
    }
}
