// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Cubemap - image with 6 layers to render a skybox

use ash::vk;
use std::rc::Rc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageMips;
use super::ImageUsage;
use crate::buffer::BufferAccess;
use crate::buffer::BufferMemory;
use crate::buffer::BufferUsage;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct Cubemap {
    memory: ImageMemory,
}

pub(crate) struct CubemapOptions<'side> {
    pub(crate) top: &'side [u8],
    pub(crate) bottom: &'side [u8],
    pub(crate) front: &'side [u8],
    pub(crate) back: &'side [u8],
    pub(crate) left: &'side [u8],
    pub(crate) right: &'side [u8],
    pub(crate) size: u32,
    pub(crate) format: ImageFormat,
}

impl Cubemap {
    pub(crate) fn new(device: &Rc<Device>, options: CubemapOptions<'_>) -> Result<Self> {
        let pixel_size = match options.format {
            ImageFormat::Srgba | ImageFormat::Rgba => 4,
            _ => panic!("unsupported cubemap format {:?}", options.format),
        };

        let size = (options.size * options.size) as usize * pixel_size;

        // create staging buffers
        let top_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        top_staging_memory.copy_from_data(options.top, size)?;

        let bottom_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        bottom_staging_memory.copy_from_data(options.bottom, size)?;

        let front_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        front_staging_memory.copy_from_data(options.front, size)?;

        let back_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        back_staging_memory.copy_from_data(options.back, size)?;

        let left_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        left_staging_memory.copy_from_data(options.left, size)?;

        let right_staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        right_staging_memory.copy_from_data(options.right, size)?;

        // create image
        let mut memory = ImageMemory::new(
            device,
            ImageMemoryOptions {
                width: options.size,
                height: options.size,
                mips: ImageMips::Log2,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                format: options.format,
                cubemap: true,
                ..Default::default()
            },
        )?;

        // copy images from staging memory
        memory.change_layout(ImageLayout::TransferDst)?;
        memory.copy_from_memory(&right_staging_memory, 0)?;
        memory.copy_from_memory(&left_staging_memory, 1)?;
        memory.copy_from_memory(&top_staging_memory, 2)?;
        memory.copy_from_memory(&bottom_staging_memory, 3)?;
        memory.copy_from_memory(&front_staging_memory, 4)?;
        memory.copy_from_memory(&back_staging_memory, 5)?;
        memory.change_layout(ImageLayout::ShaderColor)?;

        Ok(Self { memory })
    }

    pub(crate) fn add_view(&mut self) -> Result<vk::ImageView> {
        self.memory.add_view()
    }
}
