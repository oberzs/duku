// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use std::ptr;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use super::Image;
use super::ImageFormat;
use super::ImageLayout;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::device::Commands;
use crate::device::Device;
use crate::image::Msaa;
use crate::pipeline::Descriptor;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderWorld;
use crate::storage::Index;
use crate::surface::Swapchain;
use crate::vk;

// user facing framebuffer data
#[derive(Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,

    pub(crate) index: Index,

    updater: Sender<(Index, FramebufferData)>,
}

// data storage for a framebuffer
pub(crate) struct CoreFramebuffer {
    handle: vk::Framebuffer,
    render_pass: RenderPass,
    images: Vec<Image>,
    stored_index: usize,
    shader_image: Option<Image>,
    shader_index: Option<i32>,

    // resources needed for each
    // framebuffer in rendering
    world_descriptor: Descriptor,
    world_buffer: Buffer<ShaderWorld>,

    msaa: Msaa,
    width: u32,
    height: u32,

    device: Rc<Device>,
}

pub(crate) struct FramebufferData {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub(crate) struct FramebufferOptions<'formats> {
    pub(crate) attachment_formats: &'formats [ImageFormat],
    pub(crate) msaa: Msaa,
    pub(crate) depth: bool,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Framebuffer {
    pub(crate) fn new(index: Index, updater: Sender<(Index, FramebufferData)>) -> Self {
        Self {
            width: 1,
            height: 1,
            updater,
            index,
        }
    }

    pub fn update(&self) {
        let data = FramebufferData {
            width: self.width,
            height: self.height,
        };
        self.updater
            .send((self.index.clone(), data))
            .expect("bad receiver");
    }
}

impl CoreFramebuffer {
    pub(crate) fn for_swapchain(
        device: &Rc<Device>,
        swapchain: &Swapchain,
        shader_layout: &ShaderLayout,
        msaa: Msaa,
    ) -> Vec<Self> {
        let width = swapchain.width();
        let height = swapchain.height();
        let attachment_formats = &[ImageFormat::Sbgra];

        // create a framebuffer for each image in the swapchain
        swapchain
            .iter_images()
            .map(|img| {
                let render_pass = RenderPass::new(device, attachment_formats, msaa, true, true);
                let mut images: Vec<_> = render_pass
                    .attachments()
                    .map(|attachment| {
                        let handle = if attachment.is_stored() {
                            Some(img)
                        } else {
                            None
                        };

                        Image::attachment(device, &attachment, width, height, handle)
                    })
                    .collect();

                let views: Vec<_> = images.iter_mut().map(|i| i.add_view()).collect();

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

                let handle = device.create_framebuffer(&info);

                let world_buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
                let world_descriptor = shader_layout.world_set(&world_buffer);

                Self {
                    device: Rc::clone(device),
                    shader_image: None,
                    shader_index: None,
                    stored_index: 0,
                    world_buffer,
                    world_descriptor,
                    render_pass,
                    handle,
                    width,
                    height,
                    images,
                    msaa,
                }
            })
            .collect()
    }

    pub(crate) fn new(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
        options: FramebufferOptions<'_>,
    ) -> Self {
        let FramebufferOptions {
            width,
            height,
            attachment_formats,
            msaa,
            depth,
        } = options;

        let render_pass = RenderPass::new(device, attachment_formats, msaa, depth, false);

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

                Image::attachment(device, &attachment, width, height, None)
            })
            .collect();

        let views: Vec<_> = images.iter_mut().map(|i| i.add_view()).collect();

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

        let handle = device.create_framebuffer(&info);

        let world_buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let world_descriptor = shader_layout.world_set(&world_buffer);

        let mut shader_image = Image::shader(device, width, height);
        let shader_index = shader_images.add(shader_image.add_view());

        // ready image layouts
        shader_image.change_layout(ImageLayout::Undefined, ImageLayout::ShaderColor);
        images[stored_index].change_layout(
            ImageLayout::Undefined,
            match stored_format {
                Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
                _ => ImageLayout::ShaderColor,
            },
        );

        Self {
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            device: Rc::clone(device),
            world_buffer,
            world_descriptor,
            stored_index,
            render_pass,
            handle,
            width,
            height,
            images,
            msaa,
        }
    }

    pub(crate) fn update(&mut self, shader_images: &mut ShaderImages, data: FramebufferData) {
        debug_assert!(
            self.render_pass.attachments().count() == self.images.len(),
            "trying to resize swapchain framebuffer"
        );

        let FramebufferData { width, height } = data;

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

                Image::attachment(&self.device, attachment, width, height, None)
            })
            .collect();

        let views: Vec<_> = images.iter_mut().map(|i| i.add_view()).collect();

        let info = vk::FramebufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            render_pass: self.render_pass.handle(),
            attachment_count: views.len() as u32,
            p_attachments: views.as_ptr(),
            layers: 1,
            width,
            height,
        };

        shader_images.remove(self.shader_index.expect("bad texture index"));

        let mut shader_image = Image::shader(&self.device, width, height);
        let shader_index = shader_images.add(shader_image.add_view());

        // ready image layouts
        shader_image.change_layout(ImageLayout::Undefined, ImageLayout::ShaderColor);
        images[stored_index].change_layout(
            ImageLayout::Undefined,
            match stored_format {
                Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
                _ => ImageLayout::ShaderColor,
            },
        );

        // reassign new values
        self.device.destroy_framebuffer(self.handle);
        self.handle = self.device.create_framebuffer(&info);
        self.images = images;
        self.stored_index = stored_index;
        self.shader_image = Some(shader_image);
        self.shader_index = Some(shader_index);
        self.width = width;
        self.height = height;
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

    pub(crate) const fn handle(&self) -> vk::Framebuffer {
        self.handle
    }

    pub(crate) const fn render_pass(&self) -> vk::RenderPass {
        self.render_pass.handle()
    }

    pub(crate) const fn msaa(&self) -> Msaa {
        self.msaa
    }

    pub(crate) const fn width(&self) -> u32 {
        self.width
    }

    pub(crate) const fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.images[self.stored_index].get_view(0)
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &Image> {
        self.images.iter()
    }

    pub(crate) fn shader_index(&self) -> i32 {
        self.shader_index.expect("bad framebuffer")
    }

    pub(crate) fn update_world(&self, data: ShaderWorld) {
        self.world_buffer.copy_from_data(&[data]);
    }

    pub(crate) const fn world(&self) -> Descriptor {
        self.world_descriptor
    }
}

impl Drop for CoreFramebuffer {
    fn drop(&mut self) {
        self.device.destroy_framebuffer(self.handle);
    }
}
