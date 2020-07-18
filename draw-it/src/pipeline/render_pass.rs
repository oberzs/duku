// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// RenderPass - struct that structures a rendering pass

use ash::vk;
use std::sync::Arc;

use super::Attachment;
use super::AttachmentOptions;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::ImageSamples;

pub(crate) struct RenderPass {
    handle: vk::RenderPass,
    attachments: Vec<Attachment>,
    device: Arc<Device>,
}

impl RenderPass {
    pub(crate) fn new(
        device: &Arc<Device>,
        attachment_formats: &[ImageFormat],
        multisampled: bool,
        depth: bool,
        present: bool,
    ) -> Result<Self> {
        profile_scope!("new");

        debug_assert!(!present || attachment_formats.len() == 1);

        let mut depth_attachment = None;
        let mut color_attachments = vec![];
        let mut resolve_attachments = vec![];
        let mut attachment_descriptions = vec![];
        let mut attachments = vec![];

        // add depth attachment if needed
        if depth {
            let samples = if multisampled {
                device.samples()
            } else {
                ImageSamples(1)
            };

            let a = Attachment::new(AttachmentOptions {
                format: ImageFormat::Depth,
                layout: ImageLayout::Depth,
                clear: true,
                store: false,
                samples,
                index: attachments.len() as u32,
            });

            depth_attachment = Some(a.reference());
            attachment_descriptions.push(a.description());
            attachments.push(a);
        }

        // add color and resolve attachments
        for format in attachment_formats {
            debug_assert!(format.is_color());

            // base color attachment
            let layout = if present {
                ImageLayout::Present
            } else {
                ImageLayout::ShaderColor
            };

            let a = Attachment::new(AttachmentOptions {
                format: *format,
                samples: ImageSamples(1),
                clear: !multisampled,
                store: true,
                index: attachments.len() as u32,
                layout,
            });

            if multisampled {
                resolve_attachments.push(a.reference());
            } else {
                color_attachments.push(a.reference());
            }
            attachment_descriptions.push(a.description());
            attachments.push(a);

            // color multisampled attachment
            if multisampled {
                let a_msaa = Attachment::new(AttachmentOptions {
                    format: *format,
                    layout: ImageLayout::Color,
                    samples: device.samples(),
                    clear: true,
                    store: false,
                    index: attachments.len() as u32,
                });

                color_attachments.push(a_msaa.reference());
                attachment_descriptions.push(a_msaa.description());
                attachments.push(a_msaa);
            }
        }

        // create subpass dependency
        let dependency = [vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .build()];

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

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &Attachment> {
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
