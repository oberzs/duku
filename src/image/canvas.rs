// Oliver Berzs
// https://github.com/oberzs/duku

use std::ptr;

use super::Format;
use super::Image;
use super::ImageLayout;
use crate::device::Commands;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Material;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderConfig;
use crate::pipeline::Uniforms;
use crate::resources::Handle;
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
/// ```ignore
/// let canvas = duku.create_canvas(400, 400)?;
///
/// // render to canvas
/// duku.draw(&canvas, None, |target| {
///     // draw commands ...
/// });
///
/// // draw canvas on window
/// duku.draw_on_window(None, |target| {
///     target.fullscreen(&canvas);
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
    material: Option<Handle<Material>>,

    transient_images: Vec<Image>,
    stored_images: Vec<Image>,
    shader_image: Option<(u32, Image)>,
}

impl Canvas {
    pub(crate) fn for_swapchain(
        device: &Device,
        config: ShaderConfig,
        swapchain: &Swapchain,
    ) -> Vec<Self> {
        let width = swapchain.width();
        let height = swapchain.height();

        // create a texture for each image in the swapchain
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
                        let mut image =
                            Image::attachment(device, &attachment, width, height, Some(img));
                        views.push(image.add_view(device));
                        stored_images.push(image);
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

                Self {
                    shader_image: None,
                    material: None,
                    attachments,
                    transient_images,
                    stored_images,
                    render_pass,
                    framebuffer,
                    width,
                    height,
                }
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
        let render_pass = RenderPass::new(device, config, true);

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

        let mut shader_image = Image::shader(device, width, height);
        let shader_index = uniforms.add_texture(shader_image.add_view(device))?;

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

        Ok(Self {
            shader_image: Some((shader_index, shader_image)),
            material: None,
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
        debug_assert!(
            self.shader_image.is_some(),
            "trying to resize swapchain render texture"
        );

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

        let mut shader_image = Image::shader(device, self.width, self.height);
        let shader_index = self.shader_image.as_ref().expect("bad shader image").0;
        uniforms.replace_texture(shader_index, shader_image.add_view(device));

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
        device.destroy_framebuffer(self.framebuffer);
        self.framebuffer = device.create_framebuffer(&info);
        self.transient_images = transient_images;
        self.stored_images = stored_images;
        self.shader_image = Some((shader_index, shader_image));
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

    pub(crate) fn set_material(&mut self, mut material: Handle<Material>) {
        material.a[3] = self.shader_index() as f32;
        self.material = Some(material);
    }

    pub(crate) fn material(&self) -> &Handle<Material> {
        self.material.as_ref().expect("bad material")
    }

    /// Get index to be used in shader for sampling
    pub fn shader_index(&self) -> u32 {
        self.shader_image.as_ref().expect("bad shader image").0
    }
}
