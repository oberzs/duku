// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// RenderPass - struct that structures a rendering pass

use ash::vk;
use std::sync::Arc;

use super::Attachment;
use super::AttachmentOptions;
use super::AttachmentType;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::ImageSamples;
use crate::profile_scope;

pub(crate) struct RenderPass {
    handle: vk::RenderPass,
    attachments: Vec<AttachmentOptions>,
    device: Arc<Device>,
}

pub(crate) struct RenderPassOptions<'types> {
    pub(crate) attachments: &'types [AttachmentType],
    pub(crate) present: bool,
}

impl RenderPass {
    pub(crate) fn new(device: &Arc<Device>, options: RenderPassOptions<'_>) -> Result<Self> {
        profile_scope!("new");

        let mut depth_attachment = None;
        let mut color_attachments = vec![];
        let mut resolve_attachments = vec![];
        let mut attachment_descriptions = vec![];

        let mut index = 0;
        let attachments = options
            .attachments
            .iter()
            .map(|a_type| {
                let is_last = index as usize == options.attachments.len() - 1;

                match *a_type {
                    AttachmentType::Depth => {
                        let samples = if options.attachments.contains(&AttachmentType::ColorMsaa) {
                            device.samples()
                        } else {
                            ImageSamples(1)
                        };
                        let o = AttachmentOptions {
                            layout: ImageLayout::Depth,
                            format: ImageFormat::Depth,
                            clear: true,
                            store: is_last,
                            samples,
                            index,
                        };
                        index += 1;
                        let a = Attachment::new(o);
                        depth_attachment = Some(a.reference());
                        attachment_descriptions.push(a.description());
                        vec![o]
                    }
                    AttachmentType::Color => {
                        let layout = if options.present && is_last {
                            ImageLayout::Present
                        } else {
                            ImageLayout::Color
                        };
                        let o = AttachmentOptions {
                            format: ImageFormat::Bgra,
                            samples: ImageSamples(1),
                            clear: true,
                            store: is_last,
                            layout,
                            index,
                        };
                        index += 1;
                        let a = Attachment::new(o);
                        color_attachments.push(a.reference());
                        attachment_descriptions.push(a.description());
                        vec![o]
                    }
                    AttachmentType::ColorMsaa => {
                        let layout = if options.present && is_last {
                            ImageLayout::Present
                        } else {
                            ImageLayout::Color
                        };

                        // resolve attachment
                        let o = AttachmentOptions {
                            format: ImageFormat::Bgra,
                            samples: ImageSamples(1),
                            clear: false,
                            store: is_last,
                            layout,
                            index,
                        };
                        index += 1;
                        let a = Attachment::new(o);
                        resolve_attachments.push(a.reference());
                        attachment_descriptions.push(a.description());

                        // color multisampled attachment
                        let o_msaa = AttachmentOptions {
                            format: ImageFormat::Bgra,
                            layout: ImageLayout::Color,
                            samples: device.samples(),
                            clear: true,
                            store: false,
                            index,
                        };
                        index += 1;
                        let a_msaa = Attachment::new(o_msaa);
                        color_attachments.push(a_msaa.reference());
                        attachment_descriptions.push(a_msaa.description());

                        vec![o, o_msaa]
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        // create subpass dependency
        let last_type = options.attachments[options.attachments.len() - 1];
        let dependency = [match last_type {
            AttachmentType::Color | AttachmentType::ColorMsaa => vk::SubpassDependency::builder()
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
            AttachmentType::Depth => vk::SubpassDependency::builder()
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

        // create render pass
        let mut subpass_builder =
            vk::SubpassDescription::builder().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);
        if let Some(depth_a) = &depth_attachment {
            subpass_builder = subpass_builder.depth_stencil_attachment(depth_a);
        }
        if !color_attachments.is_empty() {
            subpass_builder = subpass_builder.color_attachments(&color_attachments);
        }
        if !resolve_attachments.is_empty() {
            subpass_builder = subpass_builder.resolve_attachments(&resolve_attachments);
        }
        let subpass = [subpass_builder.build()];

        let info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachment_descriptions)
            .subpasses(&subpass)
            .dependencies(&dependency);

        let handle = device.create_render_pass(&info)?;

        Ok(Self {
            device: device.clone(),
            attachments,
            handle,
        })
    }

    pub(crate) fn is_sampled(&self) -> bool {
        self.attachments
            .iter()
            .any(|a| self.device.is_msaa() && a.samples == self.device.samples())
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &AttachmentOptions> {
        self.attachments.iter()
    }

    pub(crate) fn handle(&self) -> vk::RenderPass {
        self.handle
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        self.device.destroy_render_pass(self.handle);
    }
}
