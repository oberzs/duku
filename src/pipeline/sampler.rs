// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Sampler - struct that provides image access in shader

use std::ptr;
use std::rc::Rc;

use crate::device::Device;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::vk;

pub(crate) struct Sampler {
    handle: vk::Sampler,
    device: Rc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct SamplerOptions {
    pub(crate) anisotropy: f32,
    pub(crate) wrap: TextureWrap,
    pub(crate) filter: TextureFilter,
    pub(crate) mipmaps: bool,
}

impl Sampler {
    pub(crate) fn new(device: &Rc<Device>, options: SamplerOptions) -> Self {
        let max_lod = if options.mipmaps { 16.0 } else { 0.0 };
        let mipmap_mode = if options.mipmaps {
            vk::SAMPLER_MIPMAP_MODE_LINEAR
        } else {
            vk::SAMPLER_MIPMAP_MODE_NEAREST
        };

        let max_anisotropy = if options.mipmaps {
            options.anisotropy
        } else {
            0.0
        };
        let anisotropy_enable = if options.mipmaps { vk::TRUE } else { vk::FALSE };

        let info = vk::SamplerCreateInfo {
            s_type: vk::STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            mag_filter: options.filter.flag(),
            min_filter: options.filter.flag(),
            address_mode_u: options.wrap.flag(),
            address_mode_v: options.wrap.flag(),
            address_mode_w: options.wrap.flag(),
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

        Self {
            device: Rc::clone(device),
            handle,
        }
    }

    pub(crate) const fn handle(&self) -> vk::Sampler {
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
