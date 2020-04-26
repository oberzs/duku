use ash::version::DeviceV1_0;
use ash::vk::BorderColor;
use ash::vk::CompareOp;
use ash::vk::Filter;
use ash::vk::Sampler as VkSampler;
use ash::vk::SamplerAddressMode;
use ash::vk::SamplerCreateInfo;
use ash::vk::SamplerMipmapMode;
use std::rc::Rc;
use std::rc::Weak;

use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct Sampler {
    vk: VkSampler,
    device: Weak<Device>,
}

#[derive(Debug, Copy, Clone)]
pub enum Anisotropy {
    Enabled(f32),
    Disabled,
}

impl Sampler {
    pub(crate) fn new(device: &Rc<Device>, anisotropy: Anisotropy) -> Self {
        let anisotropy_value = match anisotropy {
            Anisotropy::Enabled(value) => value,
            Anisotropy::Disabled => 0.0,
        };

        let info = SamplerCreateInfo::builder()
            .mag_filter(Filter::LINEAR)
            .min_filter(Filter::LINEAR)
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(anisotropy_value != 0.0)
            .max_anisotropy(anisotropy_value)
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
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn vk(&self) -> VkSampler {
        self.vk
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_sampler(self.vk, None);
        }
    }
}

impl Anisotropy {
    pub fn new(value: f32) -> Self {
        if value > 0.0 && value <= 16.0 {
            Self::Enabled(value)
        } else {
            Self::Disabled
        }
    }
}
