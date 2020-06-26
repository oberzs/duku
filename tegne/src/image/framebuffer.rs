// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Framebuffer - image that can be used as a render target
// also manages world uniform and camera

use ash::vk;
use std::sync::Arc;

use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageUsage;
use crate::camera::Camera;
use crate::camera::CameraType;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::AttachmentType;
use crate::pipeline::Descriptor;
use crate::pipeline::FramebufferUniform;
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
    framebuffer_uniform: Option<FramebufferUniform>,
    stored_index: usize,
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
                let camera = Camera::new(camera_type, width, height);

                Ok(Self {
                    multisampled: device.is_msaa(),
                    stored_index: 0,
                    framebuffer_uniform: None,
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
        let framebuffer_uniform =
            Some(FramebufferUniform::new(shader_layout, views[stored_index])?);
        let camera = Camera::new(camera_type, width, height);

        Ok(Self {
            stored_index,
            handle,
            render_pass,
            width,
            height,
            images,
            world_uniform,
            framebuffer_uniform,
            camera,
            multisampled,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn resize(
        &mut self,
        width: u32,
        height: u32,
        shader_layout: &ShaderLayout,
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

        let framebuffer_uniform =
            Some(FramebufferUniform::new(shader_layout, views[stored_index])?);

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
        self.stored_index = stored_index;
        self.framebuffer_uniform = framebuffer_uniform;
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

    pub(crate) fn stored_view(&self) -> vk::ImageView {
        self.images[self.stored_index].view().expect("bad code")
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = &ImageMemory> {
        self.images.iter()
    }

    pub(crate) fn world_uniform(&self) -> &WorldUniform {
        &self.world_uniform
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.framebuffer_uniform
            .as_ref()
            .expect("bad code")
            .descriptor()
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.device.destroy_framebuffer(self.handle);
    }
}
