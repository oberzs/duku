use ash::version::DeviceV1_0;
use ash::vk::AccessFlags;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineStageFlags;
use ash::vk::RenderPass as VkRenderPass;
use ash::vk::RenderPassCreateInfo;
use ash::vk::SubpassDependency;
use ash::vk::SubpassDescription;
use ash::vk::SUBPASS_EXTERNAL;
use log::debug;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

use super::Attachment;
use super::AttachmentType;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct RenderPass {
    vk: VkRenderPass,
    attachments: HashMap<AttachmentType, Attachment>,
    device: Weak<Device>,
}

impl RenderPass {
    pub(crate) fn color(device: &Rc<Device>) -> Self {
        debug!("creating color render pass");

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
                    .with_bgra_color()
                    .build(),
            );
        }

        let dependency = SubpassDependency::builder()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::empty())
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .build();

        Self::from_attachments(device, attachments, dependency)
    }

    pub(crate) fn window(device: &Rc<Device>) -> Self {
        debug!("creating window render pass");

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

        let dependency = SubpassDependency::builder()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::empty())
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .build();

        Self::from_attachments(device, attachments, dependency)
    }

    pub(crate) fn depth(device: &Rc<Device>) -> Self {
        debug!("creating offscreen depth render pass");

        let mut attachments = HashMap::new();

        attachments.insert(
            AttachmentType::Depth,
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_store()
                .with_clear()
                .build(),
        );

        let dependency = SubpassDependency::builder()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .src_access_mask(AccessFlags::empty())
            .dst_stage_mask(
                PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .dst_access_mask(
                AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                    | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .build();

        Self::from_attachments(device, attachments, dependency)
    }

    fn from_attachments(
        device: &Rc<Device>,
        attachments: HashMap<AttachmentType, Attachment>,
        dependency: SubpassDependency,
    ) -> Self {
        let dependencies = [dependency];

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

        let mut sorted_attachments = attachments.iter().map(|(_, a)| a).collect::<Vec<_>>();
        sorted_attachments.sort_by(|a, b| a.index().cmp(&b.index()));
        let vk_attachments = sorted_attachments
            .iter()
            .map(|a| a.vk())
            .collect::<Vec<_>>();

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
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn is_multisampled(&self) -> bool {
        self.attachments.contains_key(&AttachmentType::Resolve)
    }

    pub(crate) fn vk(&self) -> VkRenderPass {
        self.vk
    }

    pub(crate) fn attachments_ref(&self) -> &HashMap<AttachmentType, Attachment> {
        &self.attachments
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_render_pass(self.vk, None);
        }
    }
}
