// Oliver Berzs
// https://github.com/oberzs/draw-it

// Attachment - represents input/output image in render pass

use super::Clear;
use super::Store;
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
    store: Store,
}

impl Attachment {
    pub(crate) const fn new(
        index: u32,
        layout: ImageLayout,
        format: ImageFormat,
        msaa: Msaa,
        clear: Clear,
        store: Store,
    ) -> Self {
        let ref_layout = match layout {
            ImageLayout::Present => ImageLayout::Color,
            ImageLayout::ShaderColor => ImageLayout::Color,
            ImageLayout::ShaderDepth => ImageLayout::Depth,
            _ => layout,
        };

        let description = vk::AttachmentDescription {
            flags: 0,
            format: format.flag(),
            samples: msaa.flag(),
            stencil_load_op: vk::ATTACHMENT_LOAD_OP_DONT_CARE,
            stencil_store_op: vk::ATTACHMENT_STORE_OP_DONT_CARE,
            initial_layout: ImageLayout::Undefined.flag(),
            final_layout: layout.flag(),
            load_op: clear.flag(),
            store_op: store.flag(),
        };

        let reference = vk::AttachmentReference {
            attachment: index,
            layout: ref_layout.flag(),
        };

        Self {
            format,
            layout,
            store,
            msaa,
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

    pub(crate) fn is_stored(&self) -> bool {
        self.store == Store::Enabled
    }
}
