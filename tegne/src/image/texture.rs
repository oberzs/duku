// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Texture - simple image that can be used for rendering

use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageMips;
use super::ImageUsage;
use crate::buffer::BufferAccess;
use crate::buffer::BufferMemory;
use crate::buffer::BufferUsage;
use crate::device::Commands;
use crate::device::Device;
use crate::device::LayoutChangeOptions;
use crate::error::Result;
use crate::pipeline::ImageUniform;

pub struct Texture {
    _memory: ImageMemory,
    image_index: i32,
}

impl Texture {
    pub(crate) fn from_raw_rgb(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Self> {
        // convert image to RGBA format
        let mut rgba = vec![];
        rgba.reserve(data.len() + data.len() / 3);
        for c in data.chunks(3) {
            rgba.extend(c.iter());
            rgba.push(255);
        }
        Self::from_raw_rgba(device, uniform, &rgba, width, height)
    }

    pub(crate) fn from_raw_rgba(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let size = (width * height) as usize * 4;

        let staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        staging_memory.copy_from_data(data, size)?;

        let memory = ImageMemory::new(
            device,
            ImageMemoryOptions {
                width,
                height,
                format: ImageFormat::Rgba,
                mips: ImageMips::Log2,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                create_view: true,
                ..Default::default()
            },
        )?;

        // prepare image for data copy
        let cmd = Commands::new(device)?;
        cmd.begin()?;
        cmd.change_image_layout(
            &memory,
            LayoutChangeOptions {
                base_mip: 0,
                mip_count: memory.mip_count(),
                new_layout: ImageLayout::TransferDst,
                ..Default::default()
            },
        );
        device.submit_and_wait(cmd.end()?)?;

        memory.copy_from_memory(&staging_memory)?;
        memory.generate_mipmaps()?;

        let mut image_index = 0;
        if let Some(view) = memory.view() {
            image_index = uniform.add(view);
        }

        Ok(Self {
            _memory: memory,
            image_index,
        })
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.image_index
    }
}
