// Oliver Berzs
// https://github.com/oberzs/duku

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use std::ptr;

use super::Format;
use super::Image;
use super::ImageLayout;
use super::Msaa;
use super::Size;
use crate::device::Commands;
use crate::device::Device;
use crate::pipeline::RenderPass;
use crate::pipeline::Uniforms;
use crate::surface::Swapchain;
use crate::vk;

pub struct Framebuffer {
    handle: vk::Framebuffer,
    render_pass: RenderPass,
    images: Vec<Image>,
    stored_index: usize,
    shader_image: Option<Image>,
    shader_index: Option<u32>,

    msaa: Msaa,
    size: Size,

    should_update: bool,
}

impl Framebuffer {
    pub(crate) fn for_swapchain(device: &Device, swapchain: &Swapchain, msaa: Msaa) -> Vec<Self> {
        let size = swapchain.size();
        let attachment_formats = &[Format::Depth, Format::Sbgra];

        // create a framebuffer for each image in the swapchain
        device
            .get_swapchain_images(swapchain)
            .into_iter()
            .map(|img| {
                let render_pass = RenderPass::new(device, attachment_formats, msaa, true);
                let mut images: Vec<_> = render_pass
                    .attachments()
                    .map(|attachment| {
                        let handle = if attachment.is_stored() {
                            Some(img)
                        } else {
                            None
                        };

                        Image::attachment(device, &attachment, size, handle)
                    })
                    .collect();

                let views: Vec<_> = images.iter_mut().map(|i| i.add_view(device)).collect();

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
                    shader_index: None,
                    stored_index: 0,
                    should_update: false,
                    render_pass,
                    handle,
                    size,
                    images,
                    msaa,
                }
            })
            .collect()
    }

    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        attachment_formats: &[Format],
        msaa: Msaa,
        size: Size,
    ) -> Self {
        let render_pass = RenderPass::new(device, attachment_formats, msaa, false);

        let mut stored_format = None;
        let mut stored_index = 0;

        let mut images: Vec<_> = render_pass
            .attachments()
            .enumerate()
            .map(|(i, attachment)| {
                if attachment.is_stored() {
                    stored_format = Some(attachment.format());
                    stored_index = i;
                }

                Image::attachment(device, &attachment, size, None)
            })
            .collect();

        let views: Vec<_> = images.iter_mut().map(|i| i.add_view(device)).collect();

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
        images[stored_index].change_layout(
            device,
            ImageLayout::Undefined,
            match stored_format {
                Some(Format::Depth) => ImageLayout::ShaderDepth,
                _ => ImageLayout::ShaderColor,
            },
        );

        Self {
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            should_update: false,
            stored_index,
            render_pass,
            handle,
            size,
            images,
            msaa,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = Size::new(width, height);
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device, uniforms: &mut Uniforms) {
        debug_assert!(
            self.render_pass.attachments().count() == self.images.len(),
            "trying to resize swapchain framebuffer"
        );

        if self.should_update {
            // cleanup images
            for image in &self.images {
                image.destroy(device);
            }
            if let Some(image) = &self.shader_image {
                image.destroy(device);
            }

            // recreate framebuffer images
            let mut stored_format = None;
            let mut stored_index = 0;

            let mut images: Vec<_> = self
                .render_pass
                .attachments()
                .enumerate()
                .map(|(i, attachment)| {
                    if attachment.is_stored() {
                        stored_format = Some(attachment.format());
                        stored_index = i;
                    }

                    Image::attachment(device, attachment, self.size, None)
                })
                .collect();

            let views: Vec<_> = images.iter_mut().map(|i| i.add_view(device)).collect();

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
            uniforms.replace_image(
                self.shader_index.expect("bad shader index"),
                shader_image.add_view(device),
            );

            // ready image layouts
            shader_image.change_layout(device, ImageLayout::Undefined, ImageLayout::ShaderColor);
            images[stored_index].change_layout(
                device,
                ImageLayout::Undefined,
                match stored_format {
                    Some(Format::Depth) => ImageLayout::ShaderDepth,
                    _ => ImageLayout::ShaderColor,
                },
            );

            // reassign new values
            device.destroy_framebuffer(self.handle);
            self.handle = device.create_framebuffer(&info);
            self.images = images;
            self.stored_index = stored_index;
            self.shader_image = Some(shader_image);

            self.should_update = false;
        }
    }

    pub(crate) fn blit_to_texture(&self, cmd: &Commands) {
        if let Some(dst) = &self.shader_image {
            let src = &self.images[self.stored_index];

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
        if let Some(index) = self.shader_index {
            uniforms.remove_image(index);
        }
        for image in &self.images {
            image.destroy(device);
        }
        if let Some(image) = &self.shader_image {
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

    pub(crate) const fn msaa(&self) -> Msaa {
        self.msaa
    }

    pub(crate) const fn size(&self) -> Size {
        self.size
    }

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.images[self.stored_index].get_view(0)
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &Image> {
        self.images.iter()
    }

    pub(crate) fn shader_index(&self) -> u32 {
        self.shader_index.expect("bad framebuffer")
    }
}
