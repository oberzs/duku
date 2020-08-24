// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Shader - GPU program for execution in the renderer

use serde::Deserialize;
use std::ffi::CString;
use std::ptr;
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
use crate::vk;

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
    pub(crate) const fn new(index: Index) -> Self {
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

        let vert_module = device.create_shader_module(&data.vert);
        let frag_module = device.create_shader_module(&data.frag);
        let entry_point = CString::new("main").expect("bad code");

        // configure stages
        let stages = [
            // vertex
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: 0,
                stage: vk::SHADER_STAGE_VERTEX_BIT,
                module: vert_module,
                p_name: entry_point.as_ptr(),
                p_specialization_info: ptr::null(),
            },
            // fragment
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: 0,
                stage: vk::SHADER_STAGE_FRAGMENT_BIT,
                module: frag_module,
                p_name: entry_point.as_ptr(),
                p_specialization_info: ptr::null(),
            },
        ];

        // configure vertex input state
        let binding_descriptions = [Vertex::binding_description()];
        let attribute_descriptions = Vertex::attribute_descriptions();
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            vertex_binding_description_count: binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: binding_descriptions.as_ptr(),
            vertex_attribute_description_count: attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
        };

        // configure assembly input state
        let assembly_input_info = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            topology: shape_mode.topology(),
            primitive_restart_enable: vk::FALSE,
        };

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

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            viewport_count: viewport.len() as u32,
            p_viewports: viewport.as_ptr(),
            scissor_count: scissor.len() as u32,
            p_scissors: scissor.as_ptr(),
        };

        // configure rasterization state
        let rasterizer_state = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: shape_mode.polygon(),
            cull_mode: cull_mode.flag(),
            front_face: vk::FRONT_FACE_CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        };

        // configure msaa state
        let multisampling = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            rasterization_samples: framebuffer.msaa().flag(),
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        };

        // configure depth stencil state
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            depth_test_enable: depth_mode.test(),
            depth_write_enable: depth_mode.write(),
            depth_compare_op: vk::COMPARE_OP_LESS_OR_EQUAL,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            front: vk::StencilOpState {
                fail_op: vk::STENCIL_OP_ZERO,
                pass_op: vk::STENCIL_OP_ZERO,
                depth_fail_op: vk::STENCIL_OP_ZERO,
                compare_op: vk::COMPARE_OP_NEVER,
                compare_mask: 0,
                write_mask: 0,
                reference: 0,
            },
            back: vk::StencilOpState {
                fail_op: vk::STENCIL_OP_ZERO,
                pass_op: vk::STENCIL_OP_ZERO,
                depth_fail_op: vk::STENCIL_OP_ZERO,
                compare_op: vk::COMPARE_OP_NEVER,
                compare_mask: 0,
                write_mask: 0,
                reference: 0,
            },
            min_depth_bounds: 0.0,
            max_depth_bounds: 0.0,
        };

        // configure color blend state
        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BLEND_FACTOR_SRC_ALPHA,
            dst_color_blend_factor: vk::BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BLEND_OP_ADD,
            src_alpha_blend_factor: vk::BLEND_FACTOR_SRC_ALPHA,
            dst_alpha_blend_factor: vk::BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
            alpha_blend_op: vk::BLEND_OP_ADD,
            color_write_mask: vk::COLOR_COMPONENT_R_BIT
                | vk::COLOR_COMPONENT_G_BIT
                | vk::COLOR_COMPONENT_B_BIT
                | vk::COLOR_COMPONENT_A_BIT,
        }];

        let color_blending = vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            logic_op_enable: vk::FALSE,
            logic_op: vk::LOGIC_OP_CLEAR,
            attachment_count: color_blend_attachment.len() as u32,
            p_attachments: color_blend_attachment.as_ptr(),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
        };

        // configure dynamic state
        let dynamic_states = [
            vk::DYNAMIC_STATE_LINE_WIDTH,
            vk::DYNAMIC_STATE_SCISSOR,
            vk::DYNAMIC_STATE_VIEWPORT,
        ];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
        };

        // create pipeline
        let pipeline_info = vk::GraphicsPipelineCreateInfo {
            s_type: vk::STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vertex_input_info,
            p_input_assembly_state: &assembly_input_info,
            p_tessellation_state: ptr::null(),
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterizer_state,
            p_multisample_state: &multisampling,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blending,
            p_dynamic_state: &dynamic_state,
            layout: layout.handle(),
            render_pass: framebuffer.render_pass(),
            subpass: 0,
            base_pipeline_handle: 0,
            base_pipeline_index: 0,
        };

        let handle = device.create_pipeline(pipeline_info);

        device.destroy_shader_module(vert_module);
        device.destroy_shader_module(frag_module);

        Ok(Self {
            device: Rc::clone(device),
            handle,
        })
    }

    pub(crate) const fn handle(&self) -> vk::Pipeline {
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
