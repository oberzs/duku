// Oliver Berzs
// https://github.com/oberzs/duku

// Sampler - struct that provides image access in shader

use std::ptr;

use crate::device::Device;
use crate::image::Filter;
use crate::image::Wrap;
use crate::vk;

pub(crate) struct Sampler {
    handle: vk::Sampler,
}

impl Sampler {
    pub(crate) fn new(device: &Device, wrap: Wrap, filter: Filter, anisotropy: f32) -> Self {
        let anisotropy_enable = if anisotropy > 0.0 {
            vk::TRUE
        } else {
            vk::FALSE
        };

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
            mipmap_mode: filter.mipmap(),
            anisotropy_enable,
            max_anisotropy: anisotropy,
            max_lod: 16.0,
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
