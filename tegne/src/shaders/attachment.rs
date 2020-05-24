use ash::vk::AttachmentDescription;
use ash::vk::AttachmentLoadOp;
use ash::vk::AttachmentReference;
use ash::vk::AttachmentStoreOp;
use std::sync::Arc;

use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::ImageSamples;
use crate::instance::Device;

pub(crate) struct Attachment {
    vk: AttachmentDescription,
    reference: AttachmentReference,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct AttachmentOptions {
    pub(crate) index: u32,
    pub(crate) layout: ImageLayout,
    pub(crate) format: ImageFormat,
    pub(crate) has_samples: bool,
    pub(crate) has_clear: bool,
    pub(crate) has_store: bool,
}

impl Attachment {
    pub(crate) fn new(device: &Arc<Device>, options: AttachmentOptions) -> Self {
        let layout = if options.layout == ImageLayout::Present {
            ImageLayout::Color
        } else {
            options.layout
        };

        let clear = if options.has_clear {
            AttachmentLoadOp::CLEAR
        } else {
            AttachmentLoadOp::DONT_CARE
        };

        let store = if options.has_store {
            AttachmentStoreOp::STORE
        } else {
            AttachmentStoreOp::DONT_CARE
        };

        let samples = if options.has_samples {
            device.samples()
        } else {
            ImageSamples(1)
        };

        let vk = AttachmentDescription::builder()
            .format(options.format.flag())
            .samples(samples.flag())
            .load_op(clear)
            .store_op(store)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::Undefined.flag())
            .final_layout(options.layout.flag())
            .build();

        let reference = AttachmentReference::builder()
            .attachment(options.index)
            .layout(layout.flag())
            .build();

        Self { vk, reference }
    }

    pub(crate) fn vk(&self) -> AttachmentDescription {
        self.vk
    }

    pub(crate) fn reference(&self) -> AttachmentReference {
        self.reference
    }
}

impl Default for AttachmentOptions {
    fn default() -> Self {
        Self {
            index: 0,
            layout: ImageLayout::Undefined,
            format: ImageFormat::Depth,
            has_samples: false,
            has_clear: false,
            has_store: false,
        }
    }
}
