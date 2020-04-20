use ash::version::DeviceV1_0;
use ash::vk::BufferUsageFlags;
use ash::vk::ImageUsageFlags;
use ash::vk::MemoryPropertyFlags;
use std::cmp;
use std::rc::Rc;

use super::Image;
use super::LayoutChange;
use crate::commands::CommandRecorder;
use crate::memory::alloc;
use crate::memory::copy;
use crate::shaders::ImageUniforms;
use crate::tegne::Device;

pub enum TextureFormat {
    RGB,
    RGBA,
}

pub struct Texture {
    image: Image,
    image_index: u32,
}

impl Texture {
    pub fn white(device: &Rc<Device>, image_uniforms: &ImageUniforms) -> Self {
        Self::from_raw(
            device,
            &[255, 255, 255, 255],
            1,
            1,
            TextureFormat::RGBA,
            image_uniforms,
        )
    }

    pub fn from_raw(
        device: &Rc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        format: TextureFormat,
        image_uniforms: &ImageUniforms,
    ) -> Self {
        let mip_levels = (cmp::max(width, height) as f32).log2().floor() as u32 + 1;

        let size = width * height * 4;

        let rgba_data = match format {
            TextureFormat::RGBA => data.to_owned(),
            TextureFormat::RGB => {
                let mut rgba = vec![];
                rgba.reserve(data.len() + data.len() / 3);
                for c in data.chunks(3) {
                    rgba.extend(c.iter());
                    rgba.push(255);
                }
                rgba
            }
        };

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size as usize,
        );

        copy::data_to_buffer(&device, &rgba_data, staging_memory, size as usize);

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
        device.submit_wait(recorder.end());

        image.copy_data_from(staging_buffer);
        image.generate_mipmaps();

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        let image_index = image_uniforms.image_count();
        image_uniforms.add(image.view());

        Self { image, image_index }
    }

    pub fn image_index(&self) -> u32 {
        self.image_index
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.image.vk() == other.image.vk()
    }
}
