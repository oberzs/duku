// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Attachment - represents input/output image in render pass

use ash::vk;

use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::ImageSamples;

pub(crate) struct Attachment {
    description: vk::AttachmentDescription,
    reference: vk::AttachmentReference,
    format: ImageFormat,
    layout: ImageLayout,
    samples: ImageSamples,
    is_stored: bool,
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
            format: options.format,
            layout: options.layout,
            is_stored: options.store,
            samples: options.samples,
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

    pub(crate) fn format(&self) -> ImageFormat {
        self.format
    }

    pub(crate) fn layout(&self) -> ImageLayout {
        self.layout
    }

    pub(crate) fn samples(&self) -> ImageSamples {
        self.samples
    }

    pub(crate) fn is_stored(&self) -> bool {
        self.is_stored
    }
}
