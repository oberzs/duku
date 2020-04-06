use ash::version::DeviceV1_0;
use ash::vk::Fence as VkFence;
use ash::vk::FenceCreateFlags;
use ash::vk::FenceCreateInfo;
use std::rc::Rc;

use crate::tegne::Device;
use crate::utils::OrError;

pub struct Fence {
    vk: VkFence,
    device: Rc<Device>,
}

impl Fence {
    pub fn new(device: &Rc<Device>) -> Self {
        let info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);
        let vk = unsafe {
            device
                .logical()
                .create_fence(&info, None)
                .or_error("cannot create fence")
        };

        Self {
            vk,
            device: Rc::clone(device),
        }
    }

    pub fn wait(&self) {
        unsafe {
            self.device
                .logical()
                .wait_for_fences(&[self.vk], true, u64::max_value())
                .or_error("cannot wait for fence");
        }
    }

    pub fn reset(&self) {
        unsafe {
            self.device
                .logical()
                .reset_fences(&[self.vk])
                .or_error("cannot reset fence");
        }
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_fence(self.vk, None);
        }
    }
}
