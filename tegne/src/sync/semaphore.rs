use ash::version::DeviceV1_0;
use ash::vk::Semaphore;
use ash::vk::SemaphoreCreateInfo;
use ash::Device as LogicalDevice;

use crate::error::Result;

pub(crate) fn create(logical: &LogicalDevice) -> Result<Semaphore> {
    let info = SemaphoreCreateInfo::builder();
    let sem = unsafe { logical.create_semaphore(&info, None)? };
    Ok(sem)
}

pub(crate) fn destroy(logical: &LogicalDevice, s: Semaphore) {
    unsafe {
        logical.destroy_semaphore(s, None);
    }
}
