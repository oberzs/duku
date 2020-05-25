// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// RenderPass - struct that structures a rendering pass

use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

use super::Attachment;
use super::AttachmentOptions;
use super::DependencyType;
use crate::device::Device;
use crate::device::DeviceProperties;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageLayout;

pub(crate) struct RenderPasses {
    window: RenderPass,
    color: RenderPass,
    depth: RenderPass,
}

pub(crate) struct RenderPass {
    handle: vk::RenderPass,
    has_msaa_attachment: bool,
    device: Arc<Device>,
}

struct RenderPassOptions {
    depth_attachment: Option<Attachment>,
    color_attachment: Option<Attachment>,
    msaa_attachment: Option<Attachment>,
    dependency_type: DependencyType,
}

impl RenderPasses {
    pub(crate) fn new(device: &Arc<Device>, device_properties: &DeviceProperties) -> Result<Self> {
        let window = RenderPass::window(device, device_properties)?;
        let color = RenderPass::color(device, device_properties)?;
        let depth = RenderPass::depth(device)?;

        Ok(Self {
            window,
            color,
            depth,
        })
    }

    pub(crate) fn window(&self) -> &RenderPass {
        &self.window
    }

    pub(crate) fn color(&self) -> &RenderPass {
        &self.color
    }

    pub(crate) fn depth(&self) -> &RenderPass {
        &self.depth
    }
}

impl RenderPass {
    pub(crate) fn window(
        device: &Arc<Device>,
        device_properties: &DeviceProperties,
    ) -> Result<Self> {
        // depth
        let depth_attachment = Some(Attachment::new(AttachmentOptions {
            index: 0,
            layout: ImageLayout::Depth,
            format: ImageFormat::Depth,
            samples: device_properties.samples,
            clear: true,
            ..Default::default()
        }));

        // color
        let color_attachment = Some(Attachment::new(AttachmentOptions {
            index: 1,
            layout: ImageLayout::Present,
            format: ImageFormat::Bgra,
            clear: !device_properties.is_msaa(),
            store: true,
            ..Default::default()
        }));

        // msaa
        let msaa_attachment = if !device_properties.is_msaa() {
            None
        } else {
            Some(Attachment::new(AttachmentOptions {
                index: 2,
                layout: ImageLayout::Color,
                format: ImageFormat::Bgra,
                samples: device_properties.samples,
                clear: true,
                ..Default::default()
            }))
        };

        Self::new(
            device,
            RenderPassOptions {
                depth_attachment,
                color_attachment,
                msaa_attachment,
                dependency_type: DependencyType::Color,
            },
        )
    }

    pub(crate) fn color(
        device: &Arc<Device>,
        device_properties: &DeviceProperties,
    ) -> Result<Self> {
        // depth
        let depth_attachment = Some(Attachment::new(AttachmentOptions {
            index: 0,
            layout: ImageLayout::Depth,
            format: ImageFormat::Depth,
            samples: device_properties.samples,
            clear: true,
            ..Default::default()
        }));

        // color
        let color_attachment = Some(Attachment::new(AttachmentOptions {
            index: 1,
            layout: ImageLayout::Color,
            format: ImageFormat::Bgra,
            clear: !device_properties.is_msaa(),
            store: true,
            ..Default::default()
        }));

        // msaa
        let msaa_attachment = if !device_properties.is_msaa() {
            None
        } else {
            Some(Attachment::new(AttachmentOptions {
                index: 2,
                layout: ImageLayout::Color,
                format: ImageFormat::Bgra,
                samples: device_properties.samples,
                clear: true,
                ..Default::default()
            }))
        };

        Self::new(
            device,
            RenderPassOptions {
                depth_attachment,
                color_attachment,
                msaa_attachment,
                dependency_type: DependencyType::Color,
            },
        )
    }

    pub(crate) fn depth(device: &Arc<Device>) -> Result<Self> {
        // depth
        let depth_attachment = Some(Attachment::new(AttachmentOptions {
            index: 0,
            layout: ImageLayout::Depth,
            format: ImageFormat::Depth,
            clear: true,
            store: true,
            ..Default::default()
        }));

        Self::new(
            device,
            RenderPassOptions {
                depth_attachment,
                color_attachment: None,
                msaa_attachment: None,
                dependency_type: DependencyType::Depth,
            },
        )
    }

    fn new(device: &Arc<Device>, options: RenderPassOptions) -> Result<Self> {
        // create subpass dependency
        let dependencies = [match options.dependency_type {
            DependencyType::Color => vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::empty())
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_READ
                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
            DependencyType::Depth => vk::SubpassDependency::builder()
                .src_subpass(0)
                .dst_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(
                    vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                        | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
                )
                .src_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                .build(),
        }];

        let mut attachments = vec![];
        let mut subpass_builder =
            vk::SubpassDescription::builder().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

        // depth
        let depth_attachment;
        if let Some(attach) = &options.depth_attachment {
            depth_attachment = attach.reference();
            attachments.push(attach.description());
            subpass_builder = subpass_builder.depth_stencil_attachment(&depth_attachment);
        }

        // color
        let mut color_attachments = vec![];
        if let Some(attach) = &options.color_attachment {
            attachments.push(attach.description());
            color_attachments.push(attach.reference());
        }

        // resolve
        let mut msaa_attachments = vec![];
        if let Some(attach) = &options.msaa_attachment {
            attachments.push(attach.description());
            msaa_attachments.push(attach.reference());
        }

        let has_msaa_attachment = options.msaa_attachment.is_some();

        subpass_builder = if has_msaa_attachment {
            subpass_builder
                .color_attachments(&msaa_attachments)
                .resolve_attachments(&color_attachments)
        } else {
            subpass_builder.color_attachments(&color_attachments)
        };

        let subpasses = [subpass_builder.build()];

        let info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let handle = unsafe { device.logical().create_render_pass(&info, None)? };

        Ok(Self {
            handle,
            has_msaa_attachment,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn has_msaa_attachment(&self) -> bool {
        self.has_msaa_attachment
    }

    pub(crate) fn handle(&self) -> vk::RenderPass {
        self.handle
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_render_pass(self.handle, None);
        }
    }
}
