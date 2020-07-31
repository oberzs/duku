// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use ash::vk;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageUsage;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Attachment;
use crate::pipeline::ImageUniform;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderLayout;
use crate::pipeline::WorldUniform;
use crate::renderer::Camera;
use crate::renderer::CameraType;
use crate::surface::Swapchain;

pub struct Framebuffer {
    pub camera: Camera,

    pub(crate) world_uniform: WorldUniform,

    handle: vk::Framebuffer,
    render_pass: RenderPass,
    width: u32,
    height: u32,
    images: Vec<ImageMemory>,
    multisampled: bool,
    stored_index: usize,
    texture_image: Option<ImageMemory>,
    texture_index: Option<i32>,
    device: Arc<Device>,
}

pub(crate) struct FramebufferOptions<'formats> {
    pub(crate) attachment_formats: &'formats [ImageFormat],
    pub(crate) camera_type: CameraType,
    pub(crate) multisampled: bool,
    pub(crate) depth: bool,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Framebuffer {
    pub(crate) fn for_swapchain(
        device: &Arc<Device>,
        swapchain: &Swapchain,
        shader_layout: &ShaderLayout,
        camera_type: CameraType,
    ) -> Result<Vec<Self>> {
        let vk::Extent2D { width, height } = swapchain.extent();
        let attachment_formats = &[ImageFormat::Sbgra];

        // create a framebuffer for each image in the swapchain
        swapchain
            .iter_images()?
            .map(|img| {
                let render_pass =
                    RenderPass::new(device, attachment_formats, device.is_msaa(), true, true)?;
                let mut images = render_pass
                    .attachments()
                    .map(|attachment| {
                        let handle = if attachment.is_stored() {
                            Some(img)
                        } else {
                            None
                        };

                        create_attachment_image(device, &attachment, width, height, handle)
                    })
                    .collect::<Result<Vec<_>>>()?;

                let views = images
                    .iter_mut()
                    .map(|i| i.add_view())
                    .collect::<Result<Vec<_>>>()?;

                let info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass.handle())
                    .attachments(&views)
                    .width(width)
                    .height(height)
                    .layers(1);

                let handle = device.create_framebuffer(&info)?;

                let world_uniform = WorldUniform::new(device, shader_layout)?;
                let camera = Camera::new(camera_type, width as f32, height as f32, 100.0);

                Ok(Self {
                    multisampled: device.is_msaa(),
                    stored_index: 0,
                    texture_image: None,
                    texture_index: None,
                    handle,
                    render_pass,
                    width,
                    height,
                    images,
                    world_uniform,
                    camera,
                    device: Arc::clone(device),
                })
            })
            .collect()
    }

    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        options: FramebufferOptions<'_>,
    ) -> Result<Self> {
        let FramebufferOptions {
            width,
            height,
            attachment_formats,
            multisampled,
            depth,
            camera_type,
        } = options;

        let render_pass = RenderPass::new(device, attachment_formats, multisampled, depth, false)?;

        let mut stored_format = None;
        let mut stored_index = 0;

        let mut images = render_pass
            .attachments()
            .enumerate()
            .map(|(i, attachment)| {
                if attachment.is_stored() {
                    stored_format = Some(attachment.format());
                    stored_index = i;
                }

                create_attachment_image(device, &attachment, width, height, None)
            })
            .collect::<Result<Vec<_>>>()?;

        let views = images
            .iter_mut()
            .map(|i| i.add_view())
            .collect::<Result<Vec<_>>>()?;

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        let handle = device.create_framebuffer(&info)?;

        let world_uniform = WorldUniform::new(device, shader_layout)?;
        let camera = Camera::new(camera_type, width as f32, height as f32, 100.0);

        let mut texture_image = ImageMemory::new(
            device,
            ImageMemoryOptions {
                usage: &[
                    ImageUsage::TransferDst,
                    ImageUsage::Sampled,
                    ImageUsage::Color,
                ],
                format: ImageFormat::Sbgra,
                width,
                height,
                ..Default::default()
            },
        )?;
        let texture_index = image_uniform.add(texture_image.add_view()?);

        // ready image layouts
        texture_image.change_layout(ImageLayout::ShaderColor)?;
        images[stored_index].change_layout(match stored_format {
            Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
            _ => ImageLayout::ShaderColor,
        })?;

        Ok(Self {
            texture_image: Some(texture_image),
            texture_index: Some(texture_index),
            stored_index,
            handle,
            render_pass,
            width,
            height,
            images,
            world_uniform,
            camera,
            multisampled,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn resize(
        &mut self,
        width: u32,
        height: u32,
        image_uniform: &mut ImageUniform,
    ) -> Result<()> {
        // cannot resize swapchain framebuffer manually
        debug_assert!(self.render_pass.attachments().count() == self.images.len());

        // recreate framebuffer images
        let mut stored_format = None;
        let mut stored_index = 0;

        let mut images = self
            .render_pass
            .attachments()
            .enumerate()
            .map(|(i, attachment)| {
                if attachment.is_stored() {
                    stored_format = Some(attachment.format());
                    stored_index = i;
                }

                create_attachment_image(&self.device, attachment, width, height, None)
            })
            .collect::<Result<Vec<_>>>()?;

        let views = images
            .iter_mut()
            .map(|i| i.add_view())
            .collect::<Result<Vec<_>>>()?;

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(self.render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        image_uniform.remove(self.texture_index.expect("bad texture index"));

        let mut texture_image = ImageMemory::new(
            &self.device,
            ImageMemoryOptions {
                usage: &[
                    ImageUsage::TransferDst,
                    ImageUsage::Sampled,
                    ImageUsage::Color,
                ],
                format: ImageFormat::Sbgra,
                width,
                height,
                ..Default::default()
            },
        )?;
        let texture_index = image_uniform.add(texture_image.add_view()?);

        // ready image layouts
        texture_image.change_layout(ImageLayout::ShaderColor)?;
        images[stored_index].change_layout(match stored_format {
            Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
            _ => ImageLayout::ShaderColor,
        })?;

        // reassign new values
        self.device.destroy_framebuffer(self.handle);
        self.handle = self.device.create_framebuffer(&info)?;
        self.images = images;
        self.stored_index = stored_index;
        self.texture_image = Some(texture_image);
        self.texture_index = Some(texture_index);
        self.camera.width = width as f32;
        self.camera.height = height as f32;
        self.width = width;
        self.height = height;

        Ok(())
    }

    pub(crate) fn blit_to_texture(&mut self, cmd: vk::CommandBuffer) {
        if let Some(texture_image) = &mut self.texture_image {
            let stored_image = &mut self.images[self.stored_index];

            // prepare images for transfer
            stored_image.change_layout_sync(cmd, ImageLayout::TransferSrc);
            texture_image.change_layout_sync(cmd, ImageLayout::TransferDst);

            // blit to shader image
            let offsets = [
                vk::Offset3D::default(),
                vk::Offset3D {
                    x: self.width as i32,
                    y: self.height as i32,
                    z: 1,
                },
            ];
            let subresource = vk::ImageSubresourceLayers::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let blit = vk::ImageBlit::builder()
                .src_offsets(offsets)
                .src_subresource(subresource)
                .dst_offsets(offsets)
                .dst_subresource(subresource)
                .build();

            self.device.cmd_blit_image(
                cmd,
                stored_image.handle(),
                texture_image.handle(),
                blit,
                vk::Filter::LINEAR,
            );

            // set images back to initial state
            stored_image.change_layout_sync(cmd, ImageLayout::ShaderColor);
            texture_image.change_layout_sync(cmd, ImageLayout::ShaderColor);
        }
    }

    pub(crate) fn handle(&self) -> vk::Framebuffer {
        self.handle
    }

    pub(crate) fn render_pass(&self) -> vk::RenderPass {
        self.render_pass.handle()
    }

    pub(crate) fn multisampled(&self) -> bool {
        self.multisampled
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.images[self.stored_index].get_view(0)
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &ImageMemory> {
        self.images.iter()
    }

    pub(crate) fn texture_index(&self) -> i32 {
        self.texture_index.expect("bad framebuffer")
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.device.destroy_framebuffer(self.handle);
    }
}

fn create_attachment_image(
    device: &Arc<Device>,
    attachment: &Attachment,
    width: u32,
    height: u32,
    handle: Option<vk::Image>,
) -> Result<ImageMemory> {
    let mut usage = vec![];

    match attachment.layout() {
        ImageLayout::Color => usage.push(ImageUsage::Color),
        ImageLayout::Depth => usage.push(ImageUsage::Depth),
        ImageLayout::ShaderColor => usage.push(ImageUsage::Color),
        ImageLayout::ShaderDepth => usage.push(ImageUsage::Depth),
        _ => (),
    }

    // attachments that stay in memory can be read from
    if attachment.is_stored() {
        usage.push(ImageUsage::Sampled);

        if handle.is_none() {
            // swapchain images don't need to be transfered
            usage.push(ImageUsage::TransferSrc);
        }
    } else {
        usage.push(ImageUsage::Transient);
    }

    ImageMemory::new(
        device,
        ImageMemoryOptions {
            msaa: attachment.msaa(),
            format: attachment.format(),
            usage: &usage,
            handle,
            width,
            height,
            ..Default::default()
        },
    )
}
