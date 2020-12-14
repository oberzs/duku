// Oliver Berzs
// https://github.com/oberzs/duku

use std::ptr;

use super::Format;
use super::Image;
use super::ImageLayout;
use crate::device::Commands;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderConfig;
use crate::pipeline::Uniforms;
use crate::surface::Swapchain;
use crate::vk;

/// Texture that can be rendered to.
///
/// This collection of images can be used as the
/// target of a shader.
/// Similar to rendering to the window, but not showing
/// it on-screen.
///
/// # Examples
///
/// ```no_run
/// # use duku::Duku;
/// # let (mut duku, _) = Duku::windowed(1, 1).unwrap();
/// let canvas = duku.create_canvas(400, 400).unwrap();
///
/// // render to canvas
/// duku.draw_on_canvas(&canvas, None, |t| {
///     // draw commands ...
/// });
///
/// // draw canvas on window
/// duku.draw(None, |t| {
///     t.fullscreen(&canvas);
/// });
/// ```
pub struct Canvas {
    /// canvas image's width
    pub width: u32,
    /// canvas image's height
    pub height: u32,

    framebuffer: vk::Framebuffer,
    render_pass: RenderPass,

    attachments: Vec<Format>,
    transient_images: Vec<Image>,
    stored_images: Vec<Image>,

    base_layouts: Vec<ImageLayout>,
    shader_images: Vec<(u32, Image)>,
}

impl Canvas {
    pub(crate) fn for_swapchain(
        device: &Device,
        uniforms: &mut Uniforms,
        config: ShaderConfig,
        swapchain: &Swapchain,
    ) -> Result<Vec<Self>> {
        let width = swapchain.width();
        let height = swapchain.height();

        // create a texture for each image in the swapchain
        device
            .get_swapchain_images(swapchain)
            .into_iter()
            .map(|img| {
                let render_pass = RenderPass::new(device, config, true);

                let mut transient_images = vec![];
                let mut attachments = vec![];
                let mut views = vec![];

                let mut stored_image = None;

                for attachment in render_pass.attachments() {
                    attachments.push(attachment.format());

                    if attachment.is_stored() {
                        let mut image =
                            Image::attachment(device, &attachment, width, height, Some(img));
                        views.push(image.add_view(device));
                        stored_image = Some(image);
                    } else {
                        let mut image = Image::attachment(device, &attachment, width, height, None);
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
                    width,
                    height,
                };

                let framebuffer = device.create_framebuffer(&info);
                let stored_image = stored_image.expect("bad framebuffer");

                let mut shader_image = Image::shader(device, stored_image.format(), width, height);
                let shader_index = uniforms.add_texture(shader_image.add_view(device))?;

                let layout = if shader_image.format().is_depth() {
                    ImageLayout::ShaderDepth
                } else {
                    ImageLayout::ShaderColor
                };
                shader_image.change_layout(device, ImageLayout::Undefined, layout);

                Ok(Self {
                    shader_images: vec![(shader_index, shader_image)],
                    base_layouts: vec![ImageLayout::Present],
                    stored_images: vec![stored_image],
                    attachments,
                    transient_images,
                    render_pass,
                    framebuffer,
                    width,
                    height,
                })
            })
            .collect()
    }

    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        config: ShaderConfig,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let render_pass = RenderPass::new(device, config, false);

        let mut transient_images = vec![];
        let mut stored_images = vec![];
        let mut attachments = vec![];
        let mut views = vec![];

        for attachment in render_pass.attachments() {
            let mut image = Image::attachment(device, &attachment, width, height, None);
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
            width,
            height,
        };

        let framebuffer = device.create_framebuffer(&info);

        let mut shader_images = vec![];
        let mut base_layouts = vec![];
        for image in &stored_images {
            let base_layout = match image.format() {
                Format::Depth => ImageLayout::ShaderDepth,
                _ => ImageLayout::ShaderColor,
            };

            let mut shader_image = Image::shader(device, image.format(), width, height);
            let shader_index = uniforms.add_texture(shader_image.add_view(device))?;

            shader_image.change_layout(device, ImageLayout::Undefined, ImageLayout::ShaderColor);

            shader_images.push((shader_index, shader_image));
            base_layouts.push(base_layout);
        }

        Ok(Self {
            shader_images,
            base_layouts,
            attachments,
            transient_images,
            stored_images,
            render_pass,
            framebuffer,
            width,
            height,
        })
    }

    pub(crate) fn update(&mut self, device: &Device, uniforms: &mut Uniforms) {
        // cleanup images
        for image in &self.transient_images {
            image.destroy(device);
        }
        for image in &self.stored_images {
            image.destroy(device);
        }
        for (_, image) in &self.shader_images {
            image.destroy(device);
        }

        // recreate framebuffer images
        let mut transient_images = vec![];
        let mut stored_images = vec![];
        let mut views = vec![];

        for attachment in self.render_pass.attachments() {
            let mut image = Image::attachment(device, &attachment, self.width, self.height, None);
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
            width: self.width,
            height: self.height,
        };

        let mut shader_images = vec![];
        let mut base_layouts = vec![];
        for (i, image) in stored_images.iter().enumerate() {
            let base_layout = match image.format() {
                Format::Depth => ImageLayout::ShaderDepth,
                _ => ImageLayout::ShaderColor,
            };

            let mut shader_image = Image::shader(device, image.format(), self.width, self.height);
            let shader_index = self.shader_images[i].0;
            uniforms.replace_texture(shader_index, shader_image.add_view(device));

            shader_image.change_layout(device, ImageLayout::Undefined, ImageLayout::ShaderColor);

            shader_images.push((shader_index, shader_image));
            base_layouts.push(base_layout);
        }

        // reassign new values
        device.destroy_framebuffer(self.framebuffer);
        self.framebuffer = device.create_framebuffer(&info);
        self.transient_images = transient_images;
        self.stored_images = stored_images;
        self.shader_images = shader_images;
    }

    pub(crate) fn blit_to_texture(&self, cmd: &Commands) {
        for ((src, (_, dst)), layout) in self
            .stored_images
            .iter()
            .zip(self.shader_images.iter())
            .zip(self.base_layouts.iter())
        {
            // prepare images for transfer
            src.change_layout_sync(cmd, *layout, ImageLayout::TransferSrc);
            dst.change_layout_sync(cmd, ImageLayout::ShaderColor, ImageLayout::TransferDst);

            // blit to shader image
            cmd.blit_image(src, dst);

            // set images back to initial state
            src.change_layout_sync(cmd, ImageLayout::TransferSrc, *layout);
            dst.change_layout_sync(cmd, ImageLayout::TransferDst, ImageLayout::ShaderColor);
        }
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        for (index, image) in &self.shader_images {
            uniforms.remove_texture(*index);
            image.destroy(device);
        }
        for image in &self.transient_images {
            image.destroy(device);
        }
        for image in &self.stored_images {
            image.destroy(device);
        }
        self.render_pass.destroy(device);
        device.destroy_framebuffer(self.framebuffer);
    }

    pub(crate) const fn framebuffer(&self) -> vk::Framebuffer {
        self.framebuffer
    }

    pub(crate) const fn render_pass(&self) -> vk::RenderPass {
        self.render_pass.handle()
    }

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.stored_images[0].get_view(0)
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &Format> {
        self.attachments.iter()
    }

    /// Get index to be used in shader for sampling
    pub fn shader_index(&self, i: usize) -> Option<u32> {
        self.shader_images.get(i).map(|i| i.0)
    }
}
