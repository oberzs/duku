use std::cmp;
use std::sync::Arc;

use super::Image;
use super::ImageFormat;
use super::ImageLayout;
use super::ImageOptions;
use super::ImageUsage;
use crate::buffer::BufferAccess;
use crate::buffer::BufferMemory;
use crate::buffer::BufferUsage;
use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::LayoutChangeOptions;
use crate::shaders::ImageUniforms;

pub struct Texture {
    _image: Image,
    image_index: i32,
}

impl Texture {
    pub(crate) fn from_raw_rgb(
        device: &Arc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Result<Self> {
        let mut rgba = vec![];
        rgba.reserve(data.len() + data.len() / 3);
        for c in data.chunks(3) {
            rgba.extend(c.iter());
            rgba.push(255);
        }
        Self::from_raw_rgba(device, &rgba, width, height, image_uniforms)
    }

    pub(crate) fn from_raw_rgba(
        device: &Arc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Result<Self> {
        let mip_levels = (cmp::max(width, height) as f32).log2().floor() as u32 + 1;

        let size = width * height * 4;

        let staging_memory = BufferMemory::new(
            device,
            &[BufferUsage::TransferSrc],
            BufferAccess::Cpu,
            size as usize,
        )?;
        staging_memory.copy_from_data(data, size as usize)?;

        let image = Image::new(
            device,
            ImageOptions {
                width,
                height,
                format: ImageFormat::Rgba,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                has_view: true,
                has_mipmaps: true,
                ..Default::default()
            },
        )?;

        let cmd = Commands::new(device)?;
        cmd.begin()?;
        cmd.change_image_layout(
            &image,
            LayoutChangeOptions {
                base_mip: 0,
                mip_count: mip_levels,
                new_layout: ImageLayout::TransferDst,
                ..Default::default()
            },
        );
        device.submit_and_wait(cmd.end()?)?;

        image.copy_from_memory(&staging_memory)?;
        image.generate_mipmaps()?;

        let mut image_index = 0;
        if let Some(view) = image.view() {
            image_index = image_uniforms.add(view);
        }

        Ok(Self {
            _image: image,
            image_index,
        })
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.image_index
    }
}
