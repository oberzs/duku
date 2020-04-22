use ash::version::DeviceV1_0;
use ash::vk::AccessFlags;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineStageFlags;
use ash::vk::RenderPass as VkRenderPass;
use ash::vk::RenderPassCreateInfo;
use ash::vk::SubpassDependency;
use ash::vk::SubpassDescription;
use ash::vk::SUBPASS_EXTERNAL;
use std::collections::HashMap;
use std::rc::Rc;

use super::Attachment;
use super::AttachmentType;
use crate::instance::Device;
use crate::utils::OrError;

pub struct RenderPass {
    vk: VkRenderPass,
    attachments: HashMap<AttachmentType, Attachment>,
    device: Rc<Device>,
}

impl RenderPass {
    pub fn color_offscreen(device: &Rc<Device>) -> Self {
        let mut attachments = HashMap::new();

        attachments.insert(
            AttachmentType::Depth,
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_samples()
                .with_clear()
                .build(),
        );

        if device.is_msaa() {
            attachments.insert(
                AttachmentType::Resolve,
                Attachment::builder(device)
                    .with_index(1)
                    .with_store()
                    .with_present_layout()
                    .with_bgra_color()
                    .build(),
            );
            attachments.insert(
                AttachmentType::Color,
                Attachment::builder(device)
                    .with_index(2)
                    .with_clear()
                    .with_bgra_color()
                    .with_samples()
                    .build(),
            );
        } else {
            attachments.insert(
                AttachmentType::Color,
                Attachment::builder(device)
                    .with_index(1)
                    .with_store()
                    .with_clear()
                    .with_present_layout()
                    .build(),
            );
        }

        Self::from_attachments(device, attachments)
    }

    pub fn color_onscreen(device: &Rc<Device>) -> Self {
        let mut attachments = HashMap::new();

        attachments.insert(
            AttachmentType::Depth,
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_samples()
                .with_clear()
                .build(),
        );

        if device.is_msaa() {
            attachments.insert(
                AttachmentType::Resolve,
                Attachment::builder(device)
                    .with_index(1)
                    .with_store()
                    .with_present_layout()
                    .build(),
            );
            attachments.insert(
                AttachmentType::Color,
                Attachment::builder(device)
                    .with_index(2)
                    .with_clear()
                    .with_bgra_color()
                    .with_samples()
                    .build(),
            );
        } else {
            attachments.insert(
                AttachmentType::Color,
                Attachment::builder(device)
                    .with_index(1)
                    .with_store()
                    .with_clear()
                    .with_present_layout()
                    .build(),
            );
        }

        Self::from_attachments(device, attachments)
    }

    pub fn depth_offscreen(device: &Rc<Device>) -> Self {
        let mut attachments = HashMap::new();

        attachments.insert(
            AttachmentType::Depth,
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_samples()
                .with_store()
                .with_clear()
                .build(),
        );

        Self::from_attachments(device, attachments)
    }

    fn from_attachments(
        device: &Rc<Device>,
        attachments: HashMap<AttachmentType, Attachment>,
    ) -> Self {
        let dependencies = [SubpassDependency::builder()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::empty())
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .build()];

        let mut subpass_builder =
            SubpassDescription::builder().pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        let depth;
        if let Some(a) = attachments.get(&AttachmentType::Depth) {
            depth = a.reference();
            subpass_builder = subpass_builder.depth_stencil_attachment(&depth);
        }

        let mut color = vec![];
        if let Some(a) = attachments.get(&AttachmentType::Color) {
            color.push(a.reference());
            subpass_builder = subpass_builder.color_attachments(&color);
        }

        let mut resolve = vec![];
        if let Some(a) = attachments.get(&AttachmentType::Resolve) {
            resolve.push(a.reference());
            subpass_builder = subpass_builder.resolve_attachments(&resolve);
        }

        let subpasses = [subpass_builder.build()];

        let mut vk_attachments = vec![];
        for (_, a) in attachments.iter() {
            vk_attachments.insert(a.index() as usize, a.vk());
        }

        let info = RenderPassCreateInfo::builder()
            .attachments(&vk_attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let vk = unsafe {
            device
                .logical()
                .create_render_pass(&info, None)
                .or_error("cannot create render pass")
        };

        Self {
            vk,
            attachments,
            device: Rc::clone(device),
        }
    }

    pub fn vk(&self) -> VkRenderPass {
        self.vk
    }

    pub fn attachments_ref(&self) -> &HashMap<AttachmentType, Attachment> {
        &self.attachments
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_render_pass(self.vk, None);
        }
    }
}
