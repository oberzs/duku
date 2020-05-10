use std::rc::Rc;
use std::rc::Weak;
use tegne_math::Camera;

use crate::images::Framebuffer;
use crate::instance::Device;
use crate::instance::Target;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::ShaderLayout;

pub(crate) struct ForwardRenderer<'pass> {
    shadow_framebuffer: Framebuffer,
    depth_pass: &'pass RenderPass,
    device: Weak<Device>,
}

impl<'pass> ForwardRenderer<'pass> {
    pub(crate) fn new(
        device: &Rc<Device>,
        depth_pass: &'pass RenderPass,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
    ) -> Self {
        let shadow_framebuffer = Framebuffer::depth(
            device,
            depth_pass,
            image_uniforms,
            shader_layout,
            2048,
            2048,
        );

        Self {
            shadow_framebuffer,
            depth_pass,
            device: Rc::downgrade(device),
        }
    }

    pub fn draw(
        &self,
        framebuffer: &Framebuffer,
        render_pass: &RenderPass,
        camera: &Camera,
        target: &Target,
    ) {
    }
}
