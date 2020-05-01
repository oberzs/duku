use ash::version::DeviceV1_0;
use ash::vk::Filter;
use ash::vk::Framebuffer as VkFramebuffer;
use ash::vk::FramebufferCreateInfo;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageBlit;
use ash::vk::ImageSubresourceLayers;
use ash::vk::ImageUsageFlags;
use ash::vk::Offset3D;
use log::debug;
use std::cell::Ref;
use std::rc::Rc;
use std::rc::Weak;

use super::Image;
use crate::instance::CommandRecorder;
use crate::instance::Device;
use crate::instance::Swapchain;
use crate::shaders::AttachmentType;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldUniforms;
use crate::utils::OrError;

pub struct Framebuffer {
    vk: VkFramebuffer,
    width: u32,
    height: u32,
    attachment_images: Vec<Image>,
    shader_image: Image,
    shader_index: i32,
    world_uniforms: WorldUniforms,
    device: Weak<Device>,
}

impl Framebuffer {
    pub(crate) fn for_window(
        device: &Rc<Device>,
        swapchain: &Swapchain,
        render_pass: &RenderPass,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Vec<Self> {
        debug!("creating window framebuffers");

        let extent = device.pick_extent(width, height);
        let attachments = render_pass.attachments_ref();

        swapchain
            .iter_images()
            .map(|img| {
                let mut images = vec![];

                if attachments.contains_key(&AttachmentType::Depth) {
                    images.push(
                        Image::builder(device)
                            .with_size(extent.width, extent.height)
                            .with_samples()
                            .with_depth()
                            .with_view()
                            .with_usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                            .build(),
                    );
                }

                if attachments.contains_key(&AttachmentType::Color) {
                    images.push(
                        Image::builder(device)
                            .from_image(img)
                            .with_size(extent.width, extent.height)
                            .with_bgra_color()
                            .with_view()
                            .build(),
                    );
                }

                if attachments.contains_key(&AttachmentType::Resolve) {
                    images.push(
                        Image::builder(device)
                            .with_size(width, height)
                            .with_samples()
                            .with_bgra_color()
                            .with_view()
                            .with_usage(ImageUsageFlags::TRANSIENT_ATTACHMENT)
                            .with_usage(ImageUsageFlags::COLOR_ATTACHMENT)
                            .build(),
                    );
                }

                Self::from_images(
                    device,
                    images,
                    image_uniforms,
                    render_pass,
                    shader_layout,
                    width,
                    height,
                )
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn new(
        device: &Rc<Device>,
        render_pass: &RenderPass,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Self {
        let mut images = vec![];
        let attachments = render_pass.attachments_ref();

        if attachments.contains_key(&AttachmentType::Depth) {
            images.push(
                Image::builder(device)
                    .with_size(width, height)
                    // .with_samples() // for now
                    .with_depth()
                    .with_stencil()
                    .with_view()
                    .with_usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                    .with_usage(ImageUsageFlags::TRANSFER_SRC)
                    .build(),
            );
        }

        if attachments.contains_key(&AttachmentType::Color) {
            images.push(
                Image::builder(device)
                    .with_size(width, height)
                    .with_bgra_color()
                    .with_usage(ImageUsageFlags::COLOR_ATTACHMENT)
                    .with_usage(ImageUsageFlags::TRANSFER_SRC)
                    .with_view()
                    .build(),
            );
        }

        if attachments.contains_key(&AttachmentType::Resolve) {
            images.push(
                Image::builder(device)
                    .with_size(width, height)
                    .with_samples()
                    .with_bgra_color()
                    .with_view()
                    .with_usage(ImageUsageFlags::TRANSIENT_ATTACHMENT)
                    .with_usage(ImageUsageFlags::COLOR_ATTACHMENT)
                    .build(),
            );
        }

        Self::from_images(
            device,
            images,
            image_uniforms,
            render_pass,
            shader_layout,
            width,
            height,
        )
    }

    fn from_images(
        device: &Rc<Device>,
        images: Vec<Image>,
        image_uniforms: &ImageUniforms,
        render_pass: &RenderPass,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Self {
        let shader_image = if images.last().or_error("no images").is_depth_format() {
            Image::builder(device)
                .with_size(width, height)
                .with_depth()
                .with_view()
                .with_usage(ImageUsageFlags::TRANSFER_DST)
                .with_usage(ImageUsageFlags::SAMPLED)
                .build()
        } else {
            Image::builder(device)
                .with_size(width, height)
                .with_bgra_color()
                .with_view()
                .with_usage(ImageUsageFlags::TRANSFER_DST)
                .with_usage(ImageUsageFlags::SAMPLED)
                .build()
        };

        let recorder = CommandRecorder::new(device);
        recorder.begin_one_time();
        recorder
            .change_image_layout(&shader_image)
            .to_shader_read()
            .record();
        device.submit_buffer(recorder.end());

        let shader_index = image_uniforms.image_count() as i32;
        image_uniforms.add(shader_image.view());

        let extent = device.pick_extent(width, height);
        let attachments = images.iter().map(|i| i.view()).collect::<Vec<_>>();

        let info = FramebufferCreateInfo::builder()
            .render_pass(render_pass.vk())
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1)
            .build();

        let vk = unsafe {
            device
                .logical()
                .create_framebuffer(&info, None)
                .or_error("cannot create framebuffer")
        };

        let world_uniforms = WorldUniforms::new(device, shader_layout);

        Self {
            vk,
            width,
            height,
            shader_image,
            shader_index,
            attachment_images: images,
            world_uniforms,
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn blit_to_shader_image(&self, recorder: &Ref<'_, CommandRecorder>) {
        let image = self
            .attachment_images
            .last()
            .or_error("no attachment images");
        let is_depth = image.is_depth_format();

        if is_depth {
            recorder
                .change_image_layout(image)
                .from_depth_write()
                .to_read()
                .record();
        } else {
            recorder
                .change_image_layout(image)
                .from_color_write()
                .to_read()
                .record();
        }
        recorder
            .change_image_layout(&self.shader_image)
            .from_shader_read()
            .to_write()
            .record();

        let offsets = [
            Offset3D::default(),
            Offset3D {
                x: self.width as i32,
                y: self.height as i32,
                z: 1,
            },
        ];
        let aspect_mask = match is_depth {
            true => ImageAspectFlags::DEPTH,
            false => ImageAspectFlags::COLOR,
        };
        let subresource = ImageSubresourceLayers::builder()
            .aspect_mask(aspect_mask)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let blit = ImageBlit::builder()
            .src_offsets(offsets)
            .src_subresource(subresource)
            .dst_offsets(offsets)
            .dst_subresource(subresource)
            .build();

        let filter = match is_depth {
            true => Filter::NEAREST,
            false => Filter::LINEAR,
        };

        recorder.blit_image(image.vk(), self.shader_image.vk(), blit, filter);

        if is_depth {
            recorder
                .change_image_layout(image)
                .from_read()
                .to_depth_write()
                .record();
        } else {
            recorder
                .change_image_layout(image)
                .from_read()
                .to_color_write()
                .record();
        }
        recorder
            .change_image_layout(&self.shader_image)
            .from_write()
            .to_shader_read()
            .record();
    }

    pub(crate) fn vk(&self) -> VkFramebuffer {
        self.vk
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.shader_index
    }

    pub(crate) fn iter_attachments(&self) -> impl Iterator<Item = &Image> {
        self.attachment_images.iter()
    }

    pub(crate) fn world_uniforms(&self) -> &WorldUniforms {
        &self.world_uniforms
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_framebuffer(self.vk, None);
        }
    }
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.shader_image.vk() == other.shader_image.vk()
    }
}
