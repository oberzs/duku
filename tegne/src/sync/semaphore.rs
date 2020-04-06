use ash::version::DeviceV1_0;
use ash::vk::Semaphore as VkSemaphore;
use ash::vk::SemaphoreCreateInfo;
use std::rc::Rc;

use crate::tegne::Device;
use crate::utils::OrError;

pub struct Semaphore {
    vk: VkSemaphore,
    device: Rc<Device>,
}

impl Semaphore {
    pub fn new(device: &Rc<Device>) -> Self {
        let info = SemaphoreCreateInfo::builder();
        let vk = unsafe {
            device
                .logical()
                .create_semaphore(&info, None)
                .or_error("cannot create semaphore")
        };

        Self {
            vk,
            device: Rc::clone(device),
        }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_semaphore(self.vk, None);
        }
    }
}
