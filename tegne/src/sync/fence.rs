use ash::version::DeviceV1_0;
use ash::vk::Fence;
use ash::vk::FenceCreateFlags;
use ash::vk::FenceCreateInfo;
use ash::Device as LogicalDevice;

use crate::utils::OrError;

pub(crate) fn create(logical: &LogicalDevice) -> Fence {
    let info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);
    unsafe {
        logical
            .create_fence(&info, None)
            .or_error("cannot create fence")
    }
}

pub(crate) fn destroy(logical: &LogicalDevice, f: Fence) {
    unsafe {
        logical.destroy_fence(f, None);
    }
}

pub(crate) fn wait_for(logical: &LogicalDevice, f: Fence) {
    unsafe {
        logical
            .wait_for_fences(&[f], true, u64::max_value())
            .or_error("cannot wait for fence");
    }
}

pub(crate) fn reset(logical: &LogicalDevice, f: Fence) {
    unsafe {
        logical.reset_fences(&[f]).or_error("cannot reset fence");
    }
}
