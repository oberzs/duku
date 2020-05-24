// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Framebuffer - image that can be used as a render target
// also manages world uniforms

use ash::version::DeviceV1_0;
use ash::vk;
use log::debug;
use log::warn;
use std::cmp;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageUsage;
use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::LayoutChangeOptions;
use crate::instance::Swapchain;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::RenderPasses;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldUniforms;

pub struct Framebuffer {
    handle: vk::Framebuffer,
    width: u32,
    height: u32,
    images: Vec<ImageMemory>,
    shader_image: Option<ImageMemory>,
    shader_index: Option<i32>,
    world_uniforms: WorldUniforms,
    device: Arc<Device>,
}

impl Framebuffer {
    pub(crate) fn window(
        device: &Arc<Device>,
        swapchain: &Swapchain,
        render_passes: &RenderPasses,
        shader_layout: &ShaderLayout,
    ) -> Result<Vec<Self>> {
        debug!("creating window framebuffers");

        let extent = device.extent();
        let render_pass = render_passes.window();

        // create a framebuffer for each image in the swapchain
        swapchain
            .iter_images()?
            .map(|handle| {
                let mut images = vec![];

                // depth
                images.push(ImageMemory::new(
                    device,
                    ImageMemoryOptions {
                        width: extent.width,
                        height: extent.height,
                        format: ImageFormat::Depth,
                        usage: &[ImageUsage::Depth],
                        samples: device.samples(),
                        create_view: true,
                        ..Default::default()
                    },
                )?);

                // color
                images.push(ImageMemory::new(
                    device,
                    ImageMemoryOptions {
                        handle: Some(handle),
                        width: extent.width,
                        height: extent.height,
                        format: ImageFormat::Bgra,
                        create_view: true,
                        ..Default::default()
                    },
                )?);

                // msaa
                if device.is_msaa() {
                    images.push(ImageMemory::new(
                        device,
                        ImageMemoryOptions {
                            width: extent.width,
                            height: extent.height,
                            format: ImageFormat::Bgra,
                            usage: &[ImageUsage::Color, ImageUsage::Transient],
                            samples: device.samples(),
                            create_view: true,
                            ..Default::default()
                        },
                    )?);
                }

                let handle =
                    create_framebuffer(device, render_pass, &images, extent.width, extent.height)?;

                let world_uniforms = WorldUniforms::new(device, shader_layout)?;

                Ok(Self {
                    handle,
                    width: extent.width,
                    height: extent.height,
                    shader_image: None,
                    shader_index: None,
                    images,
                    world_uniforms,
                    device: Arc::clone(device),
                })
            })
            .collect::<Result<Vec<_>>>()
    }

    pub(crate) fn color(
        device: &Arc<Device>,
        render_passes: &RenderPasses,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut images = vec![];
        let render_pass = render_passes.color();

        // depth
        images.push(ImageMemory::new(
            device,
            ImageMemoryOptions {
                width,
                height,
                format: ImageFormat::Depth,
                usage: &[ImageUsage::Depth],
                samples: device.samples(),
                create_view: true,
                ..Default::default()
            },
        )?);

        // color
        images.push(ImageMemory::new(
            device,
            ImageMemoryOptions {
                width,
                height,
                format: ImageFormat::Bgra,
                usage: &[ImageUsage::Color, ImageUsage::TransferSrc],
                create_view: true,
                ..Default::default()
            },
        )?);

        // msaa
        if device.is_msaa() {
            images.push(ImageMemory::new(
                device,
                ImageMemoryOptions {
                    width,
                    height,
                    format: ImageFormat::Bgra,
                    usage: &[ImageUsage::Color, ImageUsage::Transient],
                    samples: device.samples(),
                    create_view: true,
                    ..Default::default()
                },
            )?);
        }

        // create image to be used in shaders
        let (shader_image, shader_index) =
            create_shader_image(device, image_uniforms, width, height, ImageFormat::Bgra)?;

        let handle = create_framebuffer(device, render_pass, &images, width, height)?;

        let world_uniforms = WorldUniforms::new(device, shader_layout)?;

        Ok(Self {
            handle,
            width,
            height,
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            images,
            world_uniforms,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn depth(
        device: &Arc<Device>,
        render_passes: &RenderPasses,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut images = vec![];
        let render_pass = render_passes.depth();

        // depth
        images.push(ImageMemory::new(
            device,
            ImageMemoryOptions {
                width,
                height,
                format: ImageFormat::DepthStencil,
                usage: &[ImageUsage::Depth, ImageUsage::TransferSrc],
                create_view: true,
                ..Default::default()
            },
        )?);

        // create image to be used in shaders
        let (shader_image, shader_index) =
            create_shader_image(device, image_uniforms, width, height, ImageFormat::Depth)?;

        let handle = create_framebuffer(device, render_pass, &images, width, height)?;

        let world_uniforms = WorldUniforms::new(device, shader_layout)?;

        Ok(Self {
            handle,
            width,
            height,
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            images,
            world_uniforms,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn update_shader_image(&self, cmd: &Commands) {
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
            cmd.change_image_layout(
                image,
                LayoutChangeOptions {
                    old_layout: layout,
                    new_layout: ImageLayout::TransferSrc,
                    ..Default::default()
                },
            );
            cmd.change_image_layout(
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

            cmd.blit_image(image.handle(), shader_image.handle(), blit, filter);

            // set images back to initial state
            cmd.change_image_layout(
                image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferSrc,
                    new_layout: layout,
                    ..Default::default()
                },
            );
            cmd.change_image_layout(
                shader_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferDst,
                    new_layout: ImageLayout::Shader,
                    ..Default::default()
                },
            );
        } else {
            warn!("trying to blit framebuffer without a shader image");
        }
    }

    pub(crate) fn handle(&self) -> vk::Framebuffer {
        self.handle
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

    pub(crate) fn world_uniforms(&self) -> &WorldUniforms {
        &self.world_uniforms
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_framebuffer(self.handle, None);
        }
    }
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.shader_image.as_ref().map(|i| i.handle())
            == other.shader_image.as_ref().map(|i| i.handle())
    }
}

fn create_shader_image(
    device: &Arc<Device>,
    uniforms: &ImageUniforms,
    width: u32,
    height: u32,
    format: ImageFormat,
) -> Result<(ImageMemory, i32)> {
    let image = ImageMemory::new(
        device,
        ImageMemoryOptions {
            width,
            height,
            format,
            usage: &[ImageUsage::Sampled, ImageUsage::TransferDst],
            create_view: true,
            ..Default::default()
        },
    )?;

    // change image layout to be used in shaders
    let cmd = Commands::new(device)?;
    cmd.begin()?;
    cmd.change_image_layout(
        &image,
        LayoutChangeOptions {
            new_layout: ImageLayout::Shader,
            ..Default::default()
        },
    );
    device.submit_and_wait(cmd.end()?)?;

    // add image to uniform descriptor
    let mut index = 0;
    if let Some(view) = image.view() {
        index = uniforms.add(view);
    }

    Ok((image, index))
}

fn create_framebuffer(
    device: &Arc<Device>,
    render_pass: &RenderPass,
    images: &[ImageMemory],
    width: u32,
    height: u32,
) -> Result<vk::Framebuffer> {
    let views = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

    let info = vk::FramebufferCreateInfo::builder()
        .render_pass(render_pass.vk())
        .attachments(&views)
        .width(width)
        .height(height)
        .layers(1);

    Ok(unsafe { device.logical().create_framebuffer(&info, None)? })
}
