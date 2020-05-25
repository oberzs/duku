// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Shader - GPU program for execution in the renderer

use ash::util::read_spv;
use ash::version::DeviceV1_0;
use ash::vk;
use std::ffi::CString;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use tar::Archive;

use super::RenderPass;
use super::ShaderLayout;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageSamples;
use crate::mesh::Vertex;

pub struct Shader {
    handle: vk::Pipeline,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub struct ShaderOptions {
    pub depth_test: bool,
    pub lines: bool,
    pub front_cull: bool,
}

impl Shader {
    pub(crate) fn new(
        device: &Arc<Device>,
        pass: &RenderPass,
        layout: &ShaderLayout,
        source: &[u8],
        options: ShaderOptions,
    ) -> Result<Self> {
        let polygon_mode = if options.lines {
            vk::PolygonMode::LINE
        } else {
            vk::PolygonMode::FILL
        };

        let front_face = if options.front_cull {
            vk::FrontFace::COUNTER_CLOCKWISE
        } else {
            vk::FrontFace::CLOCKWISE
        };

        // read shader source from archive
        let mut archive: Archive<&[u8]> = Archive::new(source);

        let mut vert_source = vec![];
        let mut frag_source = vec![];

        for file in archive.entries()? {
            let mut file = file?;

            let path = file.header().path()?.into_owned();

            if path.ends_with("vert.spv") {
                file.read_to_end(&mut vert_source)?;
            }
            if path.ends_with("frag.spv") {
                file.read_to_end(&mut frag_source)?;
            }
        }

        let vert_module = create_shader_module(device, &vert_source)?;
        let frag_module = create_shader_module(device, &frag_source)?;
        let entry_point = CString::new("main")?;

        // configure vertex stage
        let vs_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point)
            .build();

        // configure fragment stage
        let fs_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point)
            .build();

        // configure vertex input state
        let binding_descriptions = [Vertex::binding_description()];
        let attribute_descriptions = Vertex::attribute_descriptions();
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
            .build();

        // configure assembly input state
        let assembly_input_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        // configure viewport state
        let viewport = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissor = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: 1,
                height: 1,
            },
        }];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewport)
            .scissors(&scissor)
            .build();

        // configure rasterization state
        let rasterizer_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(false)
            .front_face(front_face)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .polygon_mode(polygon_mode);

        // configure msaa state
        let samples = if pass.has_msaa_attachment() {
            device.samples()
        } else {
            ImageSamples(1)
        };

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(samples.flag());

        // configure depth stencil state
        let stencil = vk::StencilOpState::builder()
            .fail_op(vk::StencilOp::KEEP)
            .pass_op(vk::StencilOp::REPLACE)
            .depth_fail_op(vk::StencilOp::KEEP)
            .compare_op(vk::CompareOp::ALWAYS)
            .compare_mask(1)
            .write_mask(1)
            .reference(1)
            .build();

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(options.depth_test)
            .depth_write_enable(options.depth_test)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(true)
            .front(stencil);

        // configure color blend state
        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD)
            .build()];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(&color_blend_attachment)
            .logic_op_enable(false)
            .build();

        // configure dynamic state
        let dynamic_states = [
            vk::DynamicState::LINE_WIDTH,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::VIEWPORT,
        ];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_states)
            .build();

        // create pipeline
        let stages = [vs_stage_info, fs_stage_info];
        let pipeline_info = [vk::GraphicsPipelineCreateInfo::builder()
            .stages(&stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&assembly_input_info)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer_state)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .depth_stencil_state(&depth_stencil_state)
            .dynamic_state(&dynamic_state)
            .layout(layout.handle())
            .render_pass(pass.handle())
            .subpass(0)
            .build()];

        let handle = unsafe {
            match device.logical().create_graphics_pipelines(
                vk::PipelineCache::null(),
                &pipeline_info,
                None,
            ) {
                Ok(ps) => ps[0],
                Err(err) => return Err(err.1.into()),
            }
        };

        unsafe {
            device.logical().destroy_shader_module(vert_module, None);
            device.logical().destroy_shader_module(frag_module, None);
        }

        Ok(Self {
            handle,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn handle(&self) -> vk::Pipeline {
        self.handle
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_pipeline(self.handle, None);
        }
    }
}

impl Default for ShaderOptions {
    fn default() -> Self {
        Self {
            depth_test: true,
            lines: false,
            front_cull: false,
        }
    }
}

fn create_shader_module(device: &Arc<Device>, source: &[u8]) -> Result<vk::ShaderModule> {
    let words = read_spv(&mut Cursor::new(&source[..]))?;
    let info = vk::ShaderModuleCreateInfo::builder().code(&words).build();
    let module = unsafe { device.logical().create_shader_module(&info, None)? };
    Ok(module)
}
