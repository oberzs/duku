use ash::version::DeviceV1_0;
use ash::vk::BufferUsageFlags;
use ash::vk::ImageUsageFlags;
use ash::vk::MemoryPropertyFlags;
use std::cmp;
use std::rc::Rc;

use super::Image;
use super::LayoutChange;
use crate::instance::CommandRecorder;
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;
use crate::shaders::ImageUniforms;

pub struct Texture {
    image: Image,
    image_index: i32,
}

impl Texture {
    pub(crate) fn from_raw_rgb(
        device: &Rc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Self {
        let mut rgba = vec![];
        rgba.reserve(data.len() + data.len() / 3);
        for c in data.chunks(3) {
            rgba.extend(c.iter());
            rgba.push(255);
        }
        Self::from_raw_rgba(device, &rgba, width, height, image_uniforms)
    }

    pub(crate) fn from_raw_rgba(
        device: &Rc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Self {
        let mip_levels = (cmp::max(width, height) as f32).log2().floor() as u32 + 1;

        let size = width * height * 4;

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size as usize,
        );

        copy::data_to_buffer(&device, data, staging_memory, size as usize);

        let image = Image::builder(device)
            .with_size(width, height)
            .with_mipmaps()
            .with_rgba_color()
            .with_view()
            .with_usage(ImageUsageFlags::TRANSFER_SRC)
            .with_usage(ImageUsageFlags::TRANSFER_DST)
            .with_usage(ImageUsageFlags::SAMPLED)
            .build();

        let recorder = CommandRecorder::new(device);
        recorder.begin_one_time();
        LayoutChange::new(&recorder, &image)
            .with_mips(0, mip_levels)
            .to_write()
            .record();
        device.submit_buffer(recorder.end());

        image.copy_data_from(staging_buffer);
        image.generate_mipmaps();

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        let image_index = image_uniforms.image_count() as i32;
        image_uniforms.add(image.view());

        Self { image, image_index }
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.image_index
    }
}
