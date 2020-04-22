use ash::version::DeviceV1_0;
use ash::vk::Semaphore;
use ash::vk::SemaphoreCreateInfo;
use ash::Device as LogicalDevice;

use crate::utils::OrError;

pub(crate) fn create(logical: &LogicalDevice) -> Semaphore {
    let info = SemaphoreCreateInfo::builder();
    unsafe {
        logical
            .create_semaphore(&info, None)
            .or_error("cannot create semaphore")
    }
}

pub(crate) fn destroy(logical: &LogicalDevice, s: Semaphore) {
    unsafe {
        logical.destroy_semaphore(s, None);
    }
}
