use ash::version::DeviceV1_0;
use ash::vk::AccessFlags;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineStageFlags;
use ash::vk::RenderPass as VkRenderPass;
use ash::vk::RenderPassCreateInfo;
use ash::vk::SubpassDependency;
use ash::vk::SubpassDescription;
use ash::vk::SUBPASS_EXTERNAL;
use std::rc::Rc;
use std::rc::Weak;

use super::Attachment;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct RenderPass {
    vk: VkRenderPass,
    has_msaa_attachment: bool,
    device: Weak<Device>,
}

struct RenderPassBuilder {
    depth_attachment: Option<Attachment>,
    color_attachment: Option<Attachment>,
    msaa_attachment: Option<Attachment>,
    dependency: Option<SubpassDependency>,
    device: Rc<Device>,
}

impl RenderPass {
    pub(crate) fn window(device: &Rc<Device>) -> Self {
        let mut builder = Self::builder(device);

        // depth
        builder.with_depth(
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_samples()
                .with_clear()
                .build(),
        );

        // color
        builder.with_color(
            Attachment::builder(device)
                .with_index(1)
                .with_store()
                .with_clear_value(!device.is_msaa())
                .with_present_layout()
                .build(),
        );

        // msaa
        if device.is_msaa() {
            builder.with_msaa(
                Attachment::builder(device)
                    .with_index(2)
                    .with_clear()
                    .with_bgra_color()
                    .with_samples()
                    .build(),
            );
        }

        builder.with_dependency(
            SubpassDependency::builder()
                .src_subpass(SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags::empty())
                .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
        );

        builder.build()
    }

    pub(crate) fn color(device: &Rc<Device>) -> Self {
        let mut builder = Self::builder(device);

        // depth
        builder.with_depth(
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_samples()
                .with_clear()
                .build(),
        );

        // color
        builder.with_color(
            Attachment::builder(device)
                .with_index(1)
                .with_store()
                .with_clear_value(!device.is_msaa())
                .with_bgra_color()
                .build(),
        );

        // msaa
        if device.is_msaa() {
            builder.with_msaa(
                Attachment::builder(device)
                    .with_index(2)
                    .with_clear()
                    .with_bgra_color()
                    .with_samples()
                    .build(),
            );
        }

        builder.with_dependency(
            SubpassDependency::builder()
                .src_subpass(SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags::empty())
                .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
        );

        builder.build()
    }

    pub(crate) fn depth(device: &Rc<Device>) -> Self {
        let mut builder = Self::builder(device);

        // depth
        builder.with_depth(
            Attachment::builder(device)
                .with_index(0)
                .with_depth()
                .with_store()
                .with_clear()
                .build(),
        );

        builder.with_dependency(
            SubpassDependency::builder()
                .src_subpass(0)
                .dst_subpass(SUBPASS_EXTERNAL)
                .src_stage_mask(
                    PipelineStageFlags::EARLY_FRAGMENT_TESTS
                        | PipelineStageFlags::LATE_FRAGMENT_TESTS,
                )
                .src_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_stage_mask(PipelineStageFlags::TRANSFER)
                .dst_access_mask(AccessFlags::TRANSFER_READ)
                .build(),
        );

        builder.build()
    }

    pub(crate) fn has_msaa_attachment(&self) -> bool {
        self.has_msaa_attachment
    }

    pub(crate) fn vk(&self) -> VkRenderPass {
        self.vk
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }

    fn builder(device: &Rc<Device>) -> RenderPassBuilder {
        RenderPassBuilder {
            depth_attachment: None,
            color_attachment: None,
            msaa_attachment: None,
            dependency: None,
            device: Rc::clone(device),
        }
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_render_pass(self.vk, None);
        }
    }
}

impl RenderPassBuilder {
    pub(crate) fn build(&self) -> RenderPass {
        let dependencies = [self.dependency.expect("subpass dependency not set")];
        let mut attachments = vec![];
        let mut subpass_builder =
            SubpassDescription::builder().pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        // depth
        let depth_attachment;
        if let Some(attach) = &self.depth_attachment {
            depth_attachment = attach.reference();
            attachments.push(attach.vk());
            subpass_builder = subpass_builder.depth_stencil_attachment(&depth_attachment);
        }

        // color
        let mut color_attachments = vec![];
        if let Some(attach) = &self.color_attachment {
            attachments.push(attach.vk());
            color_attachments.push(attach.reference());
        }

        // resolve
        let mut msaa_attachments = vec![];
        if let Some(attach) = &self.msaa_attachment {
            attachments.push(attach.vk());
            msaa_attachments.push(attach.reference());
        }

        let has_msaa_attachment = self.msaa_attachment.is_some();

        subpass_builder = if has_msaa_attachment {
            subpass_builder
                .color_attachments(&msaa_attachments)
                .resolve_attachments(&color_attachments)
        } else {
            subpass_builder.color_attachments(&color_attachments)
        };

        let subpasses = [subpass_builder.build()];

        let info = RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let vk = unsafe {
            self.device
                .logical()
                .create_render_pass(&info, None)
                .or_error("cannot create render pass")
        };

        RenderPass {
            vk,
            has_msaa_attachment,
            device: Rc::downgrade(&self.device),
        }
    }

    pub(crate) fn with_depth(&mut self, attach: Attachment) -> &mut Self {
        self.depth_attachment = Some(attach);
        self
    }

    pub(crate) fn with_color(&mut self, attach: Attachment) -> &mut Self {
        self.color_attachment = Some(attach);
        self
    }

    pub(crate) fn with_msaa(&mut self, attach: Attachment) -> &mut Self {
        self.msaa_attachment = Some(attach);
        self
    }

    pub(crate) fn with_dependency(&mut self, dependency: SubpassDependency) -> &mut Self {
        self.dependency = Some(dependency);
        self
    }
}
