// Oliver Berzs
// https://github.com/oberzs/duku

// Framebuffer - images that can be used as a render targets

use std::ptr;

use super::Format;
use super::Image;
use super::ImageLayout;
use super::Size;
use crate::device::Commands;
use crate::device::Device;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderConfig;
use crate::pipeline::Uniforms;
use crate::surface::Swapchain;
use crate::vk;

pub struct Framebuffer {
    handle: vk::Framebuffer,
    render_pass: RenderPass,
    attachments: Vec<Format>,
    size: Size,

    transient_images: Vec<Image>,
    stored_images: Vec<Image>,
    shader_image: Option<(u32, Image)>,

    should_update: bool,
}

impl Framebuffer {
    pub(crate) fn for_swapchain(
        device: &Device,
        config: ShaderConfig,
        swapchain: &Swapchain,
    ) -> Vec<Self> {
        let size = swapchain.size();

        // create a framebuffer for each image in the swapchain
        device
            .get_swapchain_images(swapchain)
            .into_iter()
            .map(|img| {
                let render_pass = RenderPass::new(device, config, true);

                let mut transient_images = vec![];
                let mut stored_images = vec![];
                let mut attachments = vec![];
                let mut views = vec![];

                for attachment in render_pass.attachments() {
                    attachments.push(attachment.format());

                    if attachment.is_stored() {
                        let mut image = Image::attachment(device, &attachment, size, Some(img));
                        views.push(image.add_view(device));
                        stored_images.push(image);
                    } else {
                        let mut image = Image::attachment(device, &attachment, size, None);
                        views.push(image.add_view(device));
                        transient_images.push(image);
                    };
                }

                let info = vk::FramebufferCreateInfo {
                    s_type: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: 0,
                    render_pass: render_pass.handle(),
                    attachment_count: views.len() as u32,
                    p_attachments: views.as_ptr(),
                    layers: 1,
                    width: size.width,
                    height: size.height,
                };

                let handle = device.create_framebuffer(&info);

                Self {
                    shader_image: None,
                    should_update: false,
                    attachments,
                    transient_images,
                    stored_images,
                    render_pass,
                    handle,
                    size,
                }
            })
            .collect()
    }

    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        config: ShaderConfig,
        size: Size,
    ) -> Self {
        let render_pass = RenderPass::new(device, config, true);

        let mut transient_images = vec![];
        let mut stored_images = vec![];
        let mut attachments = vec![];
        let mut views = vec![];

        for attachment in render_pass.attachments() {
            let mut image = Image::attachment(device, &attachment, size, None);
            views.push(image.add_view(device));
            attachments.push(attachment.format());

            if attachment.is_stored() {
                stored_images.push(image);
            } else {
                transient_images.push(image);
            };
        }

        let info = vk::FramebufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            render_pass: render_pass.handle(),
            attachment_count: views.len() as u32,
            p_attachments: views.as_ptr(),
            layers: 1,
            width: size.width,
            height: size.height,
        };

        let handle = device.create_framebuffer(&info);

        let mut shader_image = Image::shader(device, size);
        let shader_index = uniforms.add_image(shader_image.add_view(device));

        // ready image layouts
        shader_image.change_layout(device, ImageLayout::Undefined, ImageLayout::ShaderColor);
        for image in &stored_images {
            image.change_layout(
                device,
                ImageLayout::Undefined,
                match image.format() {
                    Format::Depth => ImageLayout::ShaderDepth,
                    _ => ImageLayout::ShaderColor,
                },
            );
        }

        Self {
            shader_image: Some((shader_index, shader_image)),
            should_update: false,
            attachments,
            transient_images,
            stored_images,
            render_pass,
            handle,
            size,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = Size::new(width, height);
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device, uniforms: &mut Uniforms) {
        debug_assert!(
            self.shader_image.is_some(),
            "trying to resize swapchain framebuffer"
        );

        if self.should_update {
            // cleanup images
            for image in &self.transient_images {
                image.destroy(device);
            }
            for image in &self.stored_images {
                image.destroy(device);
            }
            if let Some((_, image)) = &self.shader_image {
                image.destroy(device);
            }

            // recreate framebuffer images
            let mut transient_images = vec![];
            let mut stored_images = vec![];
            let mut views = vec![];

            for attachment in self.render_pass.attachments() {
                let mut image = Image::attachment(device, &attachment, self.size, None);
                views.push(image.add_view(device));

                if attachment.is_stored() {
                    stored_images.push(image);
                } else {
                    transient_images.push(image);
                };
            }

            let info = vk::FramebufferCreateInfo {
                s_type: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: 0,
                render_pass: self.render_pass.handle(),
                attachment_count: views.len() as u32,
                p_attachments: views.as_ptr(),
                layers: 1,
                width: self.size.width,
                height: self.size.height,
            };

            let mut shader_image = Image::shader(device, self.size);
            let shader_index = self.shader_image.as_ref().expect("bad shader image").0;
            uniforms.replace_image(shader_index, shader_image.add_view(device));

            // ready image layouts
            shader_image.change_layout(device, ImageLayout::Undefined, ImageLayout::ShaderColor);
            for image in &stored_images {
                image.change_layout(
                    device,
                    ImageLayout::Undefined,
                    match image.format() {
                        Format::Depth => ImageLayout::ShaderDepth,
                        _ => ImageLayout::ShaderColor,
                    },
                );
            }

            // reassign new values
            device.destroy_framebuffer(self.handle);
            self.handle = device.create_framebuffer(&info);
            self.transient_images = transient_images;
            self.stored_images = stored_images;
            self.shader_image = Some((shader_index, shader_image));

            self.should_update = false;
        }
    }

    pub(crate) fn blit_to_texture(&self, cmd: &Commands) {
        if let Some((_, dst)) = &self.shader_image {
            // transfer only first stored image to shader for now
            let src = &self.stored_images[0];

            // prepare images for transfer
            src.change_layout_sync(cmd, ImageLayout::ShaderColor, ImageLayout::TransferSrc);
            dst.change_layout_sync(cmd, ImageLayout::ShaderColor, ImageLayout::TransferDst);

            // blit to shader image
            cmd.blit_image(src, dst);

            // set images back to initial state
            src.change_layout_sync(cmd, ImageLayout::TransferSrc, ImageLayout::ShaderColor);
            dst.change_layout_sync(cmd, ImageLayout::TransferDst, ImageLayout::ShaderColor);
        }
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        if let Some((index, image)) = &self.shader_image {
            uniforms.remove_image(*index);
            image.destroy(device);
        }
        for image in &self.transient_images {
            image.destroy(device);
        }
        for image in &self.stored_images {
            image.destroy(device);
        }
        self.render_pass.destroy(device);
        device.destroy_framebuffer(self.handle);
    }

    pub(crate) const fn handle(&self) -> vk::Framebuffer {
        self.handle
    }

    pub(crate) const fn render_pass(&self) -> vk::RenderPass {
        self.render_pass.handle()
    }

    pub(crate) const fn size(&self) -> Size {
        self.size
    }

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.stored_images[0].get_view(0)
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &Format> {
        self.attachments.iter()
    }

    pub(crate) fn shader_index(&self) -> u32 {
        self.shader_image.as_ref().expect("bad shader image").0
    }
}
