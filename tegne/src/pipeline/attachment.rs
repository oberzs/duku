// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Attachment - represents input/output image in render pass

use ash::vk;

use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::ImageSamples;

pub(crate) struct Attachment {
    description: vk::AttachmentDescription,
    reference: vk::AttachmentReference,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct AttachmentOptions {
    pub(crate) index: u32,
    pub(crate) layout: ImageLayout,
    pub(crate) format: ImageFormat,
    pub(crate) samples: ImageSamples,
    pub(crate) clear: bool,
    pub(crate) store: bool,
}

impl Attachment {
    pub(crate) fn new(options: AttachmentOptions) -> Self {
        let layout = match options.layout {
            ImageLayout::Present => ImageLayout::Color,
            ImageLayout::ShaderColor => ImageLayout::Color,
            ImageLayout::ShaderDepth => ImageLayout::Depth,
            _ => options.layout,
        };

        let load_op = if options.clear {
            vk::AttachmentLoadOp::CLEAR
        } else {
            vk::AttachmentLoadOp::DONT_CARE
        };

        let store_op = if options.store {
            vk::AttachmentStoreOp::STORE
        } else {
            vk::AttachmentStoreOp::DONT_CARE
        };

        let description = vk::AttachmentDescription::builder()
            .format(options.format.flag())
            .samples(options.samples.flag())
            .load_op(load_op)
            .store_op(store_op)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::Undefined.flag())
            .final_layout(options.layout.flag())
            .build();

        let reference = vk::AttachmentReference::builder()
            .attachment(options.index)
            .layout(layout.flag())
            .build();

        Self {
            description,
            reference,
        }
    }

    pub(crate) fn description(&self) -> vk::AttachmentDescription {
        self.description
    }

    pub(crate) fn reference(&self) -> vk::AttachmentReference {
        self.reference
    }
}
