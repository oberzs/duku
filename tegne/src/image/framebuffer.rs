// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use ash::vk;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageUsage;
use super::LayoutChangeOptions;
use crate::camera::Camera;
use crate::camera::CameraType;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::ImageUniform;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderLayout;
use crate::pipeline::WorldUniform;
use crate::surface::Swapchain;

pub struct Framebuffer {
    pub camera: Camera,
    handle: vk::Framebuffer,
    render_pass: RenderPass,
    width: u32,
    height: u32,
    images: Vec<ImageMemory>,
    world_uniform: WorldUniform,
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
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Framebuffer {
    pub(crate) fn for_swapchain(
        device: &Arc<Device>,
        swapchain: &Swapchain,
        shader_layout: &ShaderLayout,
        options: FramebufferOptions<'_>,
    ) -> Result<Vec<Self>> {
        profile_scope!("for_swapchain");

        let vk::Extent2D { width, height } = swapchain.extent();
        let FramebufferOptions {
            attachment_formats,
            camera_type,
            ..
        } = options;

        // create a framebuffer for each image in the swapchain
        swapchain
            .iter_images()?
            .map(|img| {
                let render_pass =
                    RenderPass::new(device, attachment_formats, device.is_msaa(), true)?;
                let images = render_pass
                    .attachments()
                    .map(|a| {
                        let mut usage = vec![];

                        match a.layout {
                            ImageLayout::Color => usage.push(ImageUsage::Color),
                            ImageLayout::Depth => usage.push(ImageUsage::Depth),
                            ImageLayout::ShaderColor => usage.push(ImageUsage::Color),
                            ImageLayout::ShaderDepth => usage.push(ImageUsage::Depth),
                            _ => (),
                        }

                        let mut handle = None;
                        if a.store {
                            usage.push(ImageUsage::Sampled);
                            handle = Some(img);
                        } else {
                            usage.push(ImageUsage::Transient);
                        }

                        ImageMemory::new(
                            device,
                            ImageMemoryOptions {
                                samples: a.samples,
                                format: a.format,
                                usage: &usage,
                                create_view: true,
                                width,
                                height,
                                handle,
                                ..Default::default()
                            },
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;

                let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

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
        image_uniform: &ImageUniform,
        options: FramebufferOptions<'_>,
    ) -> Result<Self> {
        profile_scope!("new");

        let FramebufferOptions {
            width,
            height,
            attachment_formats,
            multisampled,
            camera_type,
        } = options;

        let render_pass = RenderPass::new(device, attachment_formats, multisampled, false)?;

        let mut stored_format = None;
        let mut stored_index = 0;

        let images = render_pass
            .attachments()
            .enumerate()
            .map(|(i, a)| {
                let mut usage = vec![];

                match a.layout {
                    ImageLayout::Color => usage.push(ImageUsage::Color),
                    ImageLayout::Depth => usage.push(ImageUsage::Depth),
                    ImageLayout::ShaderColor => usage.push(ImageUsage::Color),
                    ImageLayout::ShaderDepth => usage.push(ImageUsage::Depth),
                    _ => (),
                }

                // attachments that stay in memory can be read from
                if a.store {
                    usage.push(ImageUsage::Sampled);
                    usage.push(ImageUsage::TransferSrc);
                    stored_format = Some(a.format);
                    stored_index = i;
                } else {
                    usage.push(ImageUsage::Transient);
                }

                ImageMemory::new(
                    device,
                    ImageMemoryOptions {
                        samples: a.samples,
                        usage: &usage,
                        create_view: true,
                        format: a.format,
                        width,
                        height,
                        ..Default::default()
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        let handle = device.create_framebuffer(&info)?;

        let world_uniform = WorldUniform::new(device, shader_layout)?;
        let camera = Camera::new(camera_type, width as f32, height as f32, 100.0);

        let texture_image = ImageMemory::new(
            device,
            ImageMemoryOptions {
                usage: &[
                    ImageUsage::TransferDst,
                    ImageUsage::Sampled,
                    ImageUsage::Color,
                ],
                create_view: true,
                format: ImageFormat::Sbgra,
                width,
                height,
                ..Default::default()
            },
        )?;
        let texture_index = image_uniform.add(texture_image.view().expect("bad view"));

        // ready image layouts
        device.do_commands(|cmd| {
            device.cmd_change_image_layout(
                cmd,
                &images[stored_index],
                LayoutChangeOptions {
                    new_layout: match stored_format {
                        Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
                        _ => ImageLayout::ShaderColor,
                    },
                    ..Default::default()
                },
            );
            device.cmd_change_image_layout(
                cmd,
                &texture_image,
                LayoutChangeOptions {
                    new_layout: ImageLayout::ShaderColor,
                    ..Default::default()
                },
            );
            Ok(())
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
        image_uniform: &ImageUniform,
    ) -> Result<()> {
        // check if this is not a swapchain framebuffer
        if self.render_pass.attachments().count() > self.images.len() {
            panic!("bad code: trying to resize swapchain framebuffer");
        }

        // recreate framebuffer images
        let mut stored_format = None;
        let mut stored_index = 0;

        let images = self
            .render_pass
            .attachments()
            .enumerate()
            .map(|(i, a)| {
                let mut usage = vec![];

                match a.layout {
                    ImageLayout::Color => usage.push(ImageUsage::Color),
                    ImageLayout::Depth => usage.push(ImageUsage::Depth),
                    ImageLayout::ShaderColor => usage.push(ImageUsage::Color),
                    ImageLayout::ShaderDepth => usage.push(ImageUsage::Depth),
                    _ => (),
                }

                // attachments that stay in memory can be read from
                if a.store {
                    usage.push(ImageUsage::Sampled);
                    usage.push(ImageUsage::TransferSrc);
                    stored_format = Some(a.format);
                    stored_index = i;
                } else {
                    usage.push(ImageUsage::Transient);
                }

                ImageMemory::new(
                    &self.device,
                    ImageMemoryOptions {
                        samples: a.samples,
                        usage: &usage,
                        format: a.format,
                        create_view: true,
                        width,
                        height,
                        ..Default::default()
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(self.render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        image_uniform.remove(self.texture_index.expect("bad texture index"));

        let texture_image = ImageMemory::new(
            &self.device,
            ImageMemoryOptions {
                usage: &[
                    ImageUsage::TransferDst,
                    ImageUsage::Sampled,
                    ImageUsage::Color,
                ],
                create_view: true,
                format: ImageFormat::Sbgra,
                width,
                height,
                ..Default::default()
            },
        )?;
        let texture_index = image_uniform.add(texture_image.view().expect("bad view"));

        // ready image layouts
        self.device.do_commands(|cmd| {
            self.device.cmd_change_image_layout(
                cmd,
                &images[stored_index],
                LayoutChangeOptions {
                    new_layout: match stored_format {
                        Some(ImageFormat::Depth) => ImageLayout::ShaderDepth,
                        _ => ImageLayout::ShaderColor,
                    },
                    ..Default::default()
                },
            );
            self.device.cmd_change_image_layout(
                cmd,
                &texture_image,
                LayoutChangeOptions {
                    new_layout: ImageLayout::ShaderColor,
                    ..Default::default()
                },
            );
            Ok(())
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

    pub(crate) fn blit_to_texture(&self, cmd: vk::CommandBuffer) {
        if let Some(texture_image) = &self.texture_image {
            let stored_image = &self.images[self.stored_index];
            if stored_image.has_depth_format() {
                warn!("Blit to texture for depth framebuffers is not supported");
                return;
            }

            // prepare images for transfer
            self.device.cmd_change_image_layout(
                cmd,
                stored_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::ShaderColor,
                    new_layout: ImageLayout::TransferSrc,
                    ..Default::default()
                },
            );
            self.device.cmd_change_image_layout(
                cmd,
                texture_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::ShaderColor,
                    new_layout: ImageLayout::TransferDst,
                    ..Default::default()
                },
            );

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
            self.device.cmd_change_image_layout(
                cmd,
                stored_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferSrc,
                    new_layout: ImageLayout::ShaderColor,
                    ..Default::default()
                },
            );
            self.device.cmd_change_image_layout(
                cmd,
                texture_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferDst,
                    new_layout: ImageLayout::ShaderColor,
                    ..Default::default()
                },
            );
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
        self.images[self.stored_index].view().expect("bad code")
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &ImageMemory> {
        self.images.iter()
    }

    pub(crate) fn world_uniform(&self) -> &WorldUniform {
        &self.world_uniform
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
