// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// RenderPass - struct that structures a rendering pass

use std::ptr;
use std::rc::Rc;

use super::Attachment;
use super::AttachmentOptions;
use crate::device::Device;
use crate::image::ImageFormat;
use crate::image::ImageLayout;
use crate::image::Msaa;
use crate::vk;

pub(crate) struct RenderPass {
    handle: vk::RenderPass,
    attachments: Vec<Attachment>,
    device: Rc<Device>,
}

impl RenderPass {
    pub(crate) fn new(
        device: &Rc<Device>,
        attachment_formats: &[ImageFormat],
        msaa: Msaa,
        depth: bool,
        present: bool,
    ) -> Self {
        debug_assert!(
            !present || attachment_formats.len() == 1,
            "present render pass should only have 1 attachment"
        );
        debug_assert!(
            depth || !attachment_formats.is_empty(),
            "render pass should have at least 1 attachment or depth"
        );

        let multisampled = msaa != Msaa::Disabled;

        let mut depth_attachment = None;
        let mut color_attachments = vec![];
        let mut resolve_attachments = vec![];
        let mut attachment_descriptions = vec![];
        let mut attachments = vec![];

        // add depth attachment if needed
        if depth {
            let layout = if attachment_formats.is_empty() {
                ImageLayout::ShaderDepth
            } else {
                ImageLayout::Depth
            };

            let a = Attachment::new(AttachmentOptions {
                format: ImageFormat::Depth,
                store: attachment_formats.is_empty(),
                index: attachments.len() as u32,
                clear: true,
                msaa,
                layout,
            });

            depth_attachment = Some(a.reference());
            attachment_descriptions.push(a.description());
            attachments.push(a);
        }

        // add color and resolve attachments
        for format in attachment_formats {
            debug_assert!(
                format.is_color(),
                "attachment format must be a color format"
            );

            // base color attachment
            let layout = if present {
                ImageLayout::Present
            } else {
                ImageLayout::ShaderColor
            };

            let a = Attachment::new(AttachmentOptions {
                index: attachments.len() as u32,
                msaa: Msaa::Disabled,
                clear: !multisampled,
                format: *format,
                store: true,
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
                    index: attachments.len() as u32,
                    format: *format,
                    layout: ImageLayout::Color,
                    clear: true,
                    store: false,
                    msaa,
                });

                color_attachments.push(a_msaa.reference());
                attachment_descriptions.push(a_msaa.description());
                attachments.push(a_msaa);
            }
        }

        // create subpass dependency
        let dependencies = if attachment_formats.is_empty() {
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
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];
        if let Some(depth_a) = &depth_attachment {
            subpass[0].p_depth_stencil_attachment = depth_a;
        }
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
            device: Rc::clone(device),
            attachments,
            handle,
        }
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &Attachment> {
        self.attachments.iter()
    }

    pub(crate) const fn handle(&self) -> vk::RenderPass {
        self.handle
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        self.device.destroy_render_pass(self.handle);
    }
}
