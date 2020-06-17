// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use ash::vk;
use std::cmp;
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
use crate::pipeline::AttachmentType;
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
    shader_image: Option<ImageMemory>,
    shader_index: Option<i32>,
    world_uniform: WorldUniform,
    multisampled: bool,
    device: Arc<Device>,
}

pub(crate) struct FramebufferOptions<'types> {
    pub(crate) attachment_types: &'types [AttachmentType],
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
            attachment_types,
            camera_type,
            ..
        } = options;

        // create a framebuffer for each image in the swapchain
        swapchain
            .iter_images()?
            .map(|img| {
                let render_pass =
                    RenderPass::new(device, attachment_types, device.is_msaa(), true)?;
                let images = render_pass
                    .attachments()
                    .map(|a| {
                        let mut usage = vec![];

                        match a.layout {
                            ImageLayout::Color => usage.push(ImageUsage::Color),
                            ImageLayout::Depth => usage.push(ImageUsage::Depth),
                            _ => (),
                        }

                        let mut handle = None;
                        if a.store {
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
                let camera = Camera::new(camera_type, width, height);

                Ok(Self {
                    shader_image: None,
                    shader_index: None,
                    multisampled: device.is_msaa(),
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
        image_uniform: &ImageUniform,
        shader_layout: &ShaderLayout,
        options: FramebufferOptions<'_>,
    ) -> Result<Self> {
        profile_scope!("new");

        let FramebufferOptions {
            width,
            height,
            attachment_types,
            multisampled,
            camera_type,
        } = options;

        let render_pass = RenderPass::new(device, attachment_types, multisampled, false)?;

        let mut stored_format = None;

        let images = render_pass
            .attachments()
            .map(|a| {
                let mut usage = vec![];

                match a.layout {
                    ImageLayout::Color => usage.push(ImageUsage::Color),
                    ImageLayout::Depth => usage.push(ImageUsage::Depth),
                    _ => (),
                }

                // attachments that stay in memory can be read from
                if a.store {
                    usage.push(ImageUsage::TransferSrc);
                    stored_format = Some(a.format);
                } else {
                    usage.push(ImageUsage::Transient);
                }

                let format = match a.format {
                    ImageFormat::Depth if a.store => ImageFormat::DepthStencil,
                    f => f,
                };

                ImageMemory::new(
                    device,
                    ImageMemoryOptions {
                        samples: a.samples,
                        usage: &usage,
                        create_view: true,
                        width,
                        height,
                        format,
                        ..Default::default()
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        // create image to be used in shaders if needed
        let (shader_image, shader_index) = if let Some(stored) = stored_format {
            // create shader image memory
            let img = ImageMemory::new(
                device,
                ImageMemoryOptions {
                    usage: &[ImageUsage::Sampled, ImageUsage::TransferDst],
                    create_view: true,
                    format: stored,
                    width,
                    height,
                    ..Default::default()
                },
            )?;

            // change image layout to be used in shaders
            device.do_commands(|cmd| {
                device.cmd_change_image_layout(
                    cmd,
                    &img,
                    LayoutChangeOptions {
                        new_layout: ImageLayout::Shader,
                        ..Default::default()
                    },
                );
                Ok(())
            })?;

            // add image to uniform descriptor
            let mut index = 0;
            if let Some(view) = img.view() {
                index = image_uniform.add(view);
            }

            (Some(img), Some(index))
        } else {
            (None, None)
        };

        let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        let handle = device.create_framebuffer(&info)?;

        let world_uniform = WorldUniform::new(device, shader_layout)?;
        let camera = Camera::new(camera_type, width, height);

        Ok(Self {
            handle,
            render_pass,
            width,
            height,
            shader_image,
            shader_index,
            images,
            world_uniform,
            camera,
            multisampled,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn update_shader_image(&self, cmd: vk::CommandBuffer) {
        if let Some(shader_image) = &self.shader_image {
            // pick "resolve" image
            let image = &self.images[cmp::min(self.images.len() - 1, 1)];
            let has_depth = image.has_depth_format();
            let layout = if has_depth {
                ImageLayout::Depth
            } else {
                ImageLayout::Color
            };

            // prepare images for transfer
            self.device.cmd_change_image_layout(
                cmd,
                image,
                LayoutChangeOptions {
                    old_layout: layout,
                    new_layout: ImageLayout::TransferSrc,
                    ..Default::default()
                },
            );
            self.device.cmd_change_image_layout(
                cmd,
                shader_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::Shader,
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
            let aspect_mask = if has_depth {
                vk::ImageAspectFlags::DEPTH
            } else {
                vk::ImageAspectFlags::COLOR
            };
            let subresource = vk::ImageSubresourceLayers::builder()
                .aspect_mask(aspect_mask)
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

            let filter = if has_depth {
                vk::Filter::NEAREST
            } else {
                vk::Filter::LINEAR
            };

            self.device
                .cmd_blit_image(cmd, image.handle(), shader_image.handle(), blit, filter);

            // set images back to initial state
            self.device.cmd_change_image_layout(
                cmd,
                image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferSrc,
                    new_layout: layout,
                    ..Default::default()
                },
            );
            self.device.cmd_change_image_layout(
                cmd,
                shader_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferDst,
                    new_layout: ImageLayout::Shader,
                    ..Default::default()
                },
            );
        }
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
        let images = self
            .render_pass
            .attachments()
            .map(|a| {
                let mut usage = vec![];

                match a.layout {
                    ImageLayout::Color => usage.push(ImageUsage::Color),
                    ImageLayout::Depth => usage.push(ImageUsage::Depth),
                    _ => (),
                }

                // attachments that stay in memory can be read from
                if a.store {
                    usage.push(ImageUsage::TransferSrc);
                    stored_format = Some(a.format);
                } else {
                    usage.push(ImageUsage::Transient);
                }

                let format = match a.format {
                    ImageFormat::Depth if a.store => ImageFormat::DepthStencil,
                    f => f,
                };

                ImageMemory::new(
                    &self.device,
                    ImageMemoryOptions {
                        samples: a.samples,
                        usage: &usage,
                        create_view: true,
                        width,
                        height,
                        format,
                        ..Default::default()
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        // create image to be used in shaders if needed
        let (shader_image, shader_index) = if let Some(stored) = stored_format {
            // create shader image memory
            let img = ImageMemory::new(
                &self.device,
                ImageMemoryOptions {
                    usage: &[ImageUsage::Sampled, ImageUsage::TransferDst],
                    create_view: true,
                    format: stored,
                    width,
                    height,
                    ..Default::default()
                },
            )?;

            // change image layout to be used in shaders
            self.device.do_commands(|cmd| {
                self.device.cmd_change_image_layout(
                    cmd,
                    &img,
                    LayoutChangeOptions {
                        new_layout: ImageLayout::Shader,
                        ..Default::default()
                    },
                );
                Ok(())
            })?;

            // add image to uniform descriptor
            let mut index = 0;
            if let Some(view) = img.view() {
                image_uniform.remove(self.shader_index.unwrap());
                index = image_uniform.add(view);
            }

            (Some(img), Some(index))
        } else {
            (None, None)
        };

        let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(self.render_pass.handle())
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        // reassign new values
        self.device.destroy_framebuffer(self.handle);
        self.handle = self.device.create_framebuffer(&info)?;
        self.images = images;
        self.shader_image = shader_image;
        self.shader_index = shader_index;
        self.camera.width = width;
        self.camera.height = height;
        self.width = width;
        self.height = height;

        Ok(())
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

    pub(crate) fn image_index(&self) -> i32 {
        self.shader_index.unwrap_or(0)
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &ImageMemory> {
        self.images.iter()
    }

    pub(crate) fn world_uniform(&self) -> &WorldUniform {
        &self.world_uniform
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.device.destroy_framebuffer(self.handle);
    }
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.shader_image.as_ref().map(|i| i.handle())
            == other.shader_image.as_ref().map(|i| i.handle())
    }
}
