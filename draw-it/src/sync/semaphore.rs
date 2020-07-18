// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// semaphore is a GPU synchronization object

use ash::version::DeviceV1_0;
use ash::vk;
use ash::Device as LogicalDevice;

use crate::error::Result;

pub(crate) fn create(logical: &LogicalDevice) -> Result<vk::Semaphore> {
    let info = vk::SemaphoreCreateInfo::builder();
    let sem = unsafe { logical.create_semaphore(&info, None)? };
    Ok(sem)
}

pub(crate) fn destroy(logical: &LogicalDevice, s: vk::Semaphore) {
    unsafe {
        logical.destroy_semaphore(s, None);
    }
}
