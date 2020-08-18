// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Shader - GPU program for execution in the renderer

use ash::vk;
use serde::Deserialize;
use std::ffi::CString;
use std::rc::Rc;

use super::CullMode;
use super::DepthMode;
use super::ShaderLayout;
use super::ShapeMode;
use crate::device::Device;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::mesh::Vertex;
use crate::storage::Index;

// user facing texture data
#[derive(Debug)]
pub struct Shader {
    pub(crate) index: Index,
}

// GPU data storage for a shader
pub(crate) struct CoreShader {
    handle: vk::Pipeline,
    device: Rc<Device>,
}

#[derive(Deserialize)]
struct ShaderFile {
    vert: Vec<u8>,
    frag: Vec<u8>,
    depth_mode: String,
    shape_mode: String,
    cull_mode: String,
}

impl Shader {
    pub(crate) fn new(index: Index) -> Self {
        Self { index }
    }
}

impl CoreShader {
    pub(crate) fn new(
        device: &Rc<Device>,
        framebuffer: &CoreFramebuffer,
        layout: &ShaderLayout,
        source: &[u8],
    ) -> Result<Self> {
        let data: ShaderFile = bincode::deserialize(source)?;

        let depth_mode = DepthMode::from(&data.depth_mode);
        let shape_mode = ShapeMode::from(&data.shape_mode);
        let cull_mode = CullMode::from(&data.cull_mode);

        let vert_module = device.create_shader_module(&data.vert)?;
        let frag_module = device.create_shader_module(&data.frag)?;
        let entry_point = CString::new("main").expect("bad code");

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
            .topology(shape_mode.topology())
            .primitive_restart_enable(false);

        // configure viewport state
        let viewport = [vk::Viewport {
            x: 0.0,
            y: 1.0,
            width: 1.0,
            height: -1.0,
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
            .front_face(vk::FrontFace::CLOCKWISE)
            .cull_mode(cull_mode.flag())
            .polygon_mode(shape_mode.polygon())
            .line_width(1.0);

        // configure msaa state
        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(framebuffer.msaa().flag());

        // configure depth stencil state
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(depth_mode.test())
            .depth_write_enable(depth_mode.write())
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);

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
            .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
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
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
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
            .render_pass(framebuffer.render_pass())
            .subpass(0)
            .build();

        let handle = device.create_pipeline(pipeline_info)?;

        device.destroy_shader_module(vert_module);
        device.destroy_shader_module(frag_module);

        Ok(Self {
            handle,
            device: Rc::clone(device),
        })
    }

    pub(crate) fn handle(&self) -> vk::Pipeline {
        self.handle
    }
}

impl Drop for CoreShader {
    fn drop(&mut self) {
        self.device.destroy_pipeline(self.handle);
    }
}

impl PartialEq for CoreShader {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
