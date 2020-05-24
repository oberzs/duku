// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// fence is a CPU synchronization object

use ash::version::DeviceV1_0;
use ash::vk;
use ash::Device as LogicalDevice;

use crate::error::Result;

pub(crate) fn create(logical: &LogicalDevice) -> Result<vk::Fence> {
    let info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
    let fen = unsafe { logical.create_fence(&info, None)? };
    Ok(fen)
}

pub(crate) fn destroy(logical: &LogicalDevice, f: vk::Fence) {
    unsafe {
        logical.destroy_fence(f, None);
    }
}

pub(crate) fn wait_for(logical: &LogicalDevice, f: vk::Fence) -> Result<()> {
    unsafe {
        logical.wait_for_fences(&[f], true, u64::max_value())?;
    }
    Ok(())
}

pub(crate) fn reset(logical: &LogicalDevice, f: vk::Fence) -> Result<()> {
    unsafe {
        logical.reset_fences(&[f])?;
    }
    Ok(())
}
