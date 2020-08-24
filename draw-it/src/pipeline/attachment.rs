// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Attachment - represents input/output image in render pass

use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::Msaa;
use crate::vk;

pub(crate) struct Attachment {
    description: vk::AttachmentDescription,
    reference: vk::AttachmentReference,
    format: ImageFormat,
    layout: ImageLayout,
    msaa: Msaa,
    is_stored: bool,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct AttachmentOptions {
    pub(crate) index: u32,
    pub(crate) layout: ImageLayout,
    pub(crate) format: ImageFormat,
    pub(crate) msaa: Msaa,
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
            vk::ATTACHMENT_LOAD_OP_CLEAR
        } else {
            vk::ATTACHMENT_LOAD_OP_DONT_CARE
        };

        let store_op = if options.store {
            vk::ATTACHMENT_STORE_OP_STORE
        } else {
            vk::ATTACHMENT_STORE_OP_DONT_CARE
        };

        let description = vk::AttachmentDescription {
            flags: 0,
            format: options.format.flag(),
            samples: options.msaa.flag(),
            stencil_load_op: vk::ATTACHMENT_LOAD_OP_DONT_CARE,
            stencil_store_op: vk::ATTACHMENT_STORE_OP_DONT_CARE,
            initial_layout: ImageLayout::Undefined.flag(),
            final_layout: options.layout.flag(),
            load_op,
            store_op,
        };

        let reference = vk::AttachmentReference {
            attachment: options.index,
            layout: layout.flag(),
        };

        Self {
            format: options.format,
            layout: options.layout,
            is_stored: options.store,
            msaa: options.msaa,
            description,
            reference,
        }
    }

    pub(crate) const fn description(&self) -> vk::AttachmentDescription {
        self.description
    }

    pub(crate) const fn reference(&self) -> vk::AttachmentReference {
        self.reference
    }

    pub(crate) const fn format(&self) -> ImageFormat {
        self.format
    }

    pub(crate) const fn layout(&self) -> ImageLayout {
        self.layout
    }

    pub(crate) const fn msaa(&self) -> Msaa {
        self.msaa
    }

    pub(crate) const fn is_stored(&self) -> bool {
        self.is_stored
    }
}
