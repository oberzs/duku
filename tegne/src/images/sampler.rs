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

pub(crate) struct SamplerBuilder {
    anisotropy: f32,
    address_mode: SamplerAddressMode,
    filter: Filter,
    device: Rc<Device>,
}

impl Sampler {
    pub(crate) fn builder(device: &Rc<Device>) -> SamplerBuilder {
        SamplerBuilder {
            anisotropy: 0.0,
            address_mode: SamplerAddressMode::REPEAT,
            filter: Filter::LINEAR,
            device: Rc::clone(device),
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

impl SamplerBuilder {
    pub(crate) fn build(&self) -> Sampler {
        let info = SamplerCreateInfo::builder()
            .mag_filter(self.filter)
            .min_filter(self.filter)
            .address_mode_u(self.address_mode)
            .address_mode_v(self.address_mode)
            .address_mode_w(self.address_mode)
            .anisotropy_enable(self.anisotropy != 0.0)
            .max_anisotropy(self.anisotropy)
            .border_color(BorderColor::FLOAT_OPAQUE_WHITE)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(CompareOp::ALWAYS)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(16.0);

        let vk = unsafe {
            self.device
                .logical()
                .create_sampler(&info, None)
                .or_error("cannot create sampler")
        };

        Sampler {
            vk,
            device: Rc::downgrade(&self.device),
        }
    }

    pub(crate) fn with_anisotropy(&mut self, value: f32) -> &mut Self {
        self.anisotropy = value;
        self
    }

    pub(crate) fn with_clamp_mode(&mut self) -> &mut Self {
        self.address_mode = SamplerAddressMode::CLAMP_TO_BORDER;
        self
    }

    pub(crate) fn with_nearest_filter(&mut self) -> &mut Self {
        self.filter = Filter::NEAREST;
        self
    }
}
