use ash::vk::AttachmentDescription;
use ash::vk::AttachmentLoadOp;
use ash::vk::AttachmentReference;
use ash::vk::AttachmentStoreOp;
use ash::vk::Format;
use ash::vk::ImageLayout;
use ash::vk::SampleCountFlags;
use std::rc::Rc;

use crate::instance::Device;

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum AttachmentType {
    Color,
    Depth,
    Resolve,
}

pub(crate) struct Attachment {
    vk: AttachmentDescription,
    reference: AttachmentReference,
    index: u32,
}

pub(crate) struct AttachmentBuilder {
    format: Format,
    layout: ImageLayout,
    samples: SampleCountFlags,
    clear: AttachmentLoadOp,
    store: AttachmentStoreOp,
    index: u32,
    device: Rc<Device>,
}

impl Attachment {
    pub(crate) fn builder(device: &Rc<Device>) -> AttachmentBuilder {
        AttachmentBuilder {
            format: Format::D32_SFLOAT_S8_UINT,
            layout: ImageLayout::UNDEFINED,
            samples: SampleCountFlags::TYPE_1,
            clear: AttachmentLoadOp::DONT_CARE,
            store: AttachmentStoreOp::DONT_CARE,
            index: 0,
            device: Rc::clone(device),
        }
    }

    pub(crate) fn vk(&self) -> AttachmentDescription {
        self.vk
    }

    pub(crate) fn reference(&self) -> AttachmentReference {
        self.reference
    }

    pub(crate) fn index(&self) -> u32 {
        self.index
    }
}

impl<'a> AttachmentBuilder {
    pub(crate) fn with_bgra_color(&mut self) -> &mut Self {
        self.format = self.device.pick_bgra_format();
        self.layout = ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
        self
    }

    pub(crate) fn with_depth(&mut self) -> &mut Self {
        self.format = self.device.pick_depth_format();
        self.layout = ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        self
    }

    pub(crate) fn with_present_layout(&mut self) -> &mut Self {
        self.with_bgra_color();
        self.layout = ImageLayout::PRESENT_SRC_KHR;
        self
    }

    pub(crate) fn with_samples(&mut self) -> &mut Self {
        self.samples = self.device.pick_sample_count();
        self
    }

    pub(crate) fn with_clear(&mut self) -> &mut Self {
        self.clear = AttachmentLoadOp::CLEAR;
        self
    }

    pub(crate) fn with_store(&mut self) -> &mut Self {
        self.store = AttachmentStoreOp::STORE;
        self
    }

    pub(crate) fn with_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }

    pub(crate) fn build(&self) -> Attachment {
        let vk = AttachmentDescription::builder()
            .format(self.format)
            .samples(self.samples)
            .load_op(self.clear)
            .store_op(self.store)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(self.layout)
            .build();

        let reference = AttachmentReference::builder()
            .attachment(self.index)
            .layout(self.layout)
            .build();

        Attachment {
            vk,
            reference,
            index: self.index,
        }
    }
}
