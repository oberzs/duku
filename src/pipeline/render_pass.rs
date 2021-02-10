// Oliver Berzs
// https://github.com/oberzs/duku

// RenderPass - struct that structures a rendering pass

use std::ptr;

use super::Attachment;
use super::ShaderConfig;
use super::Store;
use crate::device::Device;
use crate::image::Format;
use crate::image::ImageLayout;
use crate::image::Msaa;
use crate::vk;

pub(crate) struct RenderPass {
    handle: vk::RenderPass,
    attachments: Vec<Attachment>,
    color_attachments: u32,
}

impl RenderPass {
    pub(crate) fn new(device: &Device, config: ShaderConfig, present: bool) -> Self {
        let multisampled = config.msaa != Msaa::Disabled;
        let only_depth = config.outputs == 0;

        let mut attachment_descriptions = vec![];
        let mut attachments = vec![];

        // add depth attachment if needed
        let layout = if only_depth {
            ImageLayout::ShaderDepth
        } else {
            ImageLayout::Depth
        };

        let a = Attachment::new(
            attachments.len() as u32,
            layout,
            Format::Depth,
            config.msaa,
            Store::from(only_depth),
        );

        let depth_attachment = a.reference();
        attachment_descriptions.push(a.description());
        attachments.push(a);

        // add color and resolve attachments
        let mut color_attachments = vec![];
        let mut resolve_attachments = vec![];
        for _ in 0..config.outputs {
            // base color attachment
            let layout = if present {
                ImageLayout::Present
            } else {
                ImageLayout::Color
            };

            let a = Attachment::new(
                attachments.len() as u32,
                layout,
                Format::Bgra,
                Msaa::Disabled,
                Store::Enabled,
            );

            if multisampled {
                resolve_attachments.push(a.reference());
            } else {
                color_attachments.push(a.reference());
            }
            attachment_descriptions.push(a.description());
            attachments.push(a);

            // color multisampled attachment
            if multisampled {
                let a_msaa = Attachment::new(
                    attachments.len() as u32,
                    ImageLayout::Color,
                    Format::Bgra,
                    config.msaa,
                    Store::Disabled,
                );

                color_attachments.push(a_msaa.reference());
                attachment_descriptions.push(a_msaa.description());
                attachments.push(a_msaa);
            }
        }

        // create subpass dependency
        let dependencies = if only_depth {
            // depth pass
            [
                // start of render pass dependency
                vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    dst_subpass: 0,
                    src_stage_mask: vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
                    dst_stage_mask: vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
                    src_access_mask: 0,
                    dst_access_mask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                    dependency_flags: vk::DEPENDENCY_BY_REGION_BIT,
                },
                // end of render pass dependency
                vk::SubpassDependency {
                    src_subpass: 0,
                    dst_subpass: vk::SUBPASS_EXTERNAL,
                    src_stage_mask: vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
                    dst_stage_mask: vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                    src_access_mask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                    dst_access_mask: 0,
                    dependency_flags: vk::DEPENDENCY_BY_REGION_BIT,
                },
            ]
        } else {
            // color pass
            [
                // start of render pass dependency
                vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    dst_subpass: 0,
                    src_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                    dst_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                    src_access_mask: 0,
                    dst_access_mask: vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                    dependency_flags: vk::DEPENDENCY_BY_REGION_BIT,
                },
                // end of render pass dependency
                vk::SubpassDependency {
                    src_subpass: 0,
                    dst_subpass: vk::SUBPASS_EXTERNAL,
                    src_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                    dst_stage_mask: vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                    src_access_mask: vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                    dst_access_mask: 0,
                    dependency_flags: vk::DEPENDENCY_BY_REGION_BIT,
                },
            ]
        };

        // create render pass
        let mut subpass = [vk::SubpassDescription {
            flags: 0,
            pipeline_bind_point: vk::PIPELINE_BIND_POINT_GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: 0,
            p_color_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: &depth_attachment,
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];

        if !color_attachments.is_empty() {
            subpass[0].color_attachment_count = color_attachments.len() as u32;
            subpass[0].p_color_attachments = color_attachments.as_ptr();
        }
        if !resolve_attachments.is_empty() {
            subpass[0].p_resolve_attachments = resolve_attachments.as_ptr();
        }

        let info = vk::RenderPassCreateInfo {
            s_type: vk::STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            attachment_count: attachment_descriptions.len() as u32,
            p_attachments: attachment_descriptions.as_ptr(),
            subpass_count: subpass.len() as u32,
            p_subpasses: subpass.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
        };

        let handle = device.create_render_pass(&info);

        Self {
            color_attachments: color_attachments.len() as u32,
            attachments,
            handle,
        }
    }

    pub(crate) fn destroy(&self, device: &Device) {
        device.destroy_render_pass(self.handle);
    }

    pub(crate) const fn color_attachments(&self) -> u32 {
        self.color_attachments
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &Attachment> {
        self.attachments.iter()
    }

    pub(crate) const fn handle(&self) -> vk::RenderPass {
        self.handle
    }
}
