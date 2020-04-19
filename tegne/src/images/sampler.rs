use ash::version::DeviceV1_0;
use ash::vk::BorderColor;
use ash::vk::CompareOp;
use ash::vk::Filter;
use ash::vk::Sampler as VkSampler;
use ash::vk::SamplerAddressMode;
use ash::vk::SamplerCreateInfo;
use ash::vk::SamplerMipmapMode;
use std::rc::Rc;

use crate::tegne::Device;
use crate::utils::error;
use crate::utils::OrError;

pub struct Sampler {
    vk: VkSampler,
    device: Rc<Device>,
}

impl Sampler {
    pub fn new(device: &Rc<Device>, anisotropy: u8) -> Self {
        if anisotropy > 16 {
            error("invalid anisotropy value");
        }

        let info = SamplerCreateInfo::builder()
            .mag_filter(Filter::LINEAR)
            .min_filter(Filter::LINEAR)
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(anisotropy != 0)
            .max_anisotropy(f32::from(anisotropy))
            .border_color(BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(CompareOp::ALWAYS)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(16.0);

        let vk = unsafe {
            device
                .logical()
                .create_sampler(&info, None)
                .or_error("cannot create sampler")
        };

        Self {
            vk,
            device: Rc::clone(device),
        }
    }

    pub fn vk(&self) -> VkSampler {
        self.vk
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_sampler(self.vk, None);
        }
    }
}
