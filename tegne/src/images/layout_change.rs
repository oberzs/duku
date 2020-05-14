use ash::vk::AccessFlags;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageMemoryBarrier;
use ash::vk::ImageSubresourceRange;
use ash::vk::PipelineStageFlags;
use ash::vk::QUEUE_FAMILY_IGNORED;

use super::Image;
use super::ImageLayout;
use crate::error::Result;
use crate::instance::Commands;

pub(crate) struct LayoutChange<'a> {
    cmd: &'a Commands,
    image: &'a Image,
    old_layout: ImageLayout,
    new_layout: ImageLayout,
    src_access: AccessFlags,
    dst_access: AccessFlags,
    src_stage: PipelineStageFlags,
    dst_stage: PipelineStageFlags,
    base_mip: u32,
    mip_count: u32,
}

impl<'a> LayoutChange<'a> {
    pub(crate) fn new(cmd: &'a Commands, image: &'a Image) -> Self {
        Self {
            cmd,
            image,
            old_layout: ImageLayout::Undefined,
            new_layout: ImageLayout::Undefined,
            src_access: AccessFlags::default(),
            dst_access: AccessFlags::default(),
            src_stage: PipelineStageFlags::TOP_OF_PIPE,
            dst_stage: PipelineStageFlags::TOP_OF_PIPE,
            base_mip: 0,
            mip_count: 1,
        }
    }

    pub(crate) fn change_from_read(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::TransferSrc;
        self.src_access = AccessFlags::TRANSFER_READ;
        self.src_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn change_from_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::TransferDst;
        self.src_access = AccessFlags::TRANSFER_WRITE;
        self.src_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn change_from_shader_read(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::Shader;
        self.src_access = AccessFlags::SHADER_READ;
        self.src_stage = PipelineStageFlags::FRAGMENT_SHADER;
        self
    }

    pub(crate) fn change_from_color_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::Color;
        self.src_access = AccessFlags::COLOR_ATTACHMENT_WRITE;
        self.src_stage = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        self
    }

    pub(crate) fn change_from_depth_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::Depth;
        self.src_access = AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
        self.src_stage =
            PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS;
        self
    }

    pub(crate) fn change_to_read(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::TransferSrc;
        self.dst_access = AccessFlags::TRANSFER_READ;
        self.dst_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn change_to_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::TransferDst;
        self.dst_access = AccessFlags::TRANSFER_WRITE;
        self.dst_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn change_to_shader_read(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::Shader;
        self.dst_access = AccessFlags::SHADER_READ;
        self.dst_stage = PipelineStageFlags::FRAGMENT_SHADER;
        self
    }

    pub(crate) fn change_to_color_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::Color;
        self.dst_access = AccessFlags::COLOR_ATTACHMENT_WRITE;
        self.dst_stage = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        self
    }

    pub(crate) fn change_to_depth_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::Depth;
        self.dst_access = AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
        self.dst_stage =
            PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS;
        self
    }

    pub(crate) fn with_mips(&mut self, base: u32, count: u32) -> &mut Self {
        self.base_mip = base;
        self.mip_count = count;
        self
    }

    pub(crate) fn record(&self) -> Result<()> {
        let aspect_mask = if self.image.is_depth_format() {
            ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL
        } else {
            ImageAspectFlags::COLOR
        };

        let subresource = ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_array_layer(0)
            .base_mip_level(self.base_mip)
            .layer_count(1)
            .level_count(self.mip_count)
            .build();
        let barrier = ImageMemoryBarrier::builder()
            .src_queue_family_index(QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(QUEUE_FAMILY_IGNORED)
            .subresource_range(subresource)
            .image(self.image.vk())
            .old_layout(self.old_layout.flag())
            .new_layout(self.new_layout.flag())
            .src_access_mask(self.src_access)
            .dst_access_mask(self.dst_access)
            .build();

        self.cmd
            .set_pipeline_barrier(barrier, self.src_stage, self.dst_stage)?;

        Ok(())
    }
}
