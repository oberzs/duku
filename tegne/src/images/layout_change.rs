use ash::vk::AccessFlags;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageLayout;
use ash::vk::ImageMemoryBarrier;
use ash::vk::ImageSubresourceRange;
use ash::vk::PipelineStageFlags;
use ash::vk::QUEUE_FAMILY_IGNORED;

use super::Image;
use crate::instance::CommandRecorder;

pub(crate) struct LayoutChange<'a> {
    recorder: &'a CommandRecorder,
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
    pub(crate) fn new(recorder: &'a CommandRecorder, image: &'a Image) -> Self {
        Self {
            recorder,
            image,
            old_layout: ImageLayout::UNDEFINED,
            new_layout: ImageLayout::UNDEFINED,
            src_access: AccessFlags::default(),
            dst_access: AccessFlags::default(),
            src_stage: PipelineStageFlags::TOP_OF_PIPE,
            dst_stage: PipelineStageFlags::TOP_OF_PIPE,
            base_mip: 0,
            mip_count: 1,
        }
    }

    pub(crate) fn from_read(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::TRANSFER_SRC_OPTIMAL;
        self.src_access = AccessFlags::TRANSFER_READ;
        self.src_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn from_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::TRANSFER_DST_OPTIMAL;
        self.src_access = AccessFlags::TRANSFER_WRITE;
        self.src_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn from_shader_read(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        self.src_access = AccessFlags::SHADER_READ;
        self.src_stage = PipelineStageFlags::FRAGMENT_SHADER;
        self
    }

    pub(crate) fn from_color_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
        self.src_access = AccessFlags::COLOR_ATTACHMENT_WRITE;
        self.src_stage = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        self
    }

    pub(crate) fn from_depth_write(&mut self) -> &mut Self {
        self.old_layout = ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        self.src_access = AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
        self.src_stage =
            PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS;
        self
    }

    pub(crate) fn to_read(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::TRANSFER_SRC_OPTIMAL;
        self.dst_access = AccessFlags::TRANSFER_READ;
        self.dst_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn to_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::TRANSFER_DST_OPTIMAL;
        self.dst_access = AccessFlags::TRANSFER_WRITE;
        self.dst_stage = PipelineStageFlags::TRANSFER;
        self
    }

    pub(crate) fn to_shader_read(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        self.dst_access = AccessFlags::SHADER_READ;
        self.dst_stage = PipelineStageFlags::FRAGMENT_SHADER;
        self
    }

    pub(crate) fn to_color_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
        self.dst_access = AccessFlags::COLOR_ATTACHMENT_WRITE;
        self.dst_stage = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        self
    }

    pub(crate) fn to_depth_write(&mut self) -> &mut Self {
        self.new_layout = ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
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

    pub(crate) fn record(&self) {
        let aspect_mask = match self.image.is_depth_format() {
            true => ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL,
            false => ImageAspectFlags::COLOR,
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
            .old_layout(self.old_layout)
            .new_layout(self.new_layout)
            .src_access_mask(self.src_access)
            .dst_access_mask(self.dst_access)
            .build();

        self.recorder
            .set_pipeline_barrier(barrier, self.src_stage, self.dst_stage);
    }
}
