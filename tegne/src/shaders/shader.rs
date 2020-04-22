use ash::util::read_spv;
use ash::version::DeviceV1_0;
use ash::vk::BlendFactor;
use ash::vk::BlendOp;
use ash::vk::ColorComponentFlags;
use ash::vk::CompareOp;
use ash::vk::CullModeFlags;
use ash::vk::DynamicState;
use ash::vk::Extent2D;
use ash::vk::FrontFace;
use ash::vk::GraphicsPipelineCreateInfo;
use ash::vk::Offset2D;
use ash::vk::Pipeline;
use ash::vk::PipelineCache;
use ash::vk::PipelineColorBlendAttachmentState;
use ash::vk::PipelineColorBlendStateCreateInfo;
use ash::vk::PipelineDepthStencilStateCreateInfo;
use ash::vk::PipelineDynamicStateCreateInfo;
use ash::vk::PipelineInputAssemblyStateCreateInfo;
use ash::vk::PipelineMultisampleStateCreateInfo;
use ash::vk::PipelineRasterizationStateCreateInfo;
use ash::vk::PipelineShaderStageCreateInfo;
use ash::vk::PipelineVertexInputStateCreateInfo;
use ash::vk::PipelineViewportStateCreateInfo;
use ash::vk::PolygonMode;
use ash::vk::PrimitiveTopology;
use ash::vk::Rect2D;
use ash::vk::ShaderModule;
use ash::vk::ShaderModuleCreateInfo;
use ash::vk::ShaderStageFlags;
use ash::vk::StencilOp;
use ash::vk::StencilOpState;
use ash::vk::Viewport;
use std::io::Cursor;
use std::rc::Rc;

use super::RenderPass;
use super::ShaderLayout;
use crate::instance::Device;
use crate::model::Vertex;
use crate::utils::cstring;
use crate::utils::OrError;

pub(crate) enum FragmentMode {
    Fill,
    Lines,
}

pub(crate) enum CullMode {
    Back,
    Front,
}

pub(crate) enum Depth {
    Enabled,
    Disabled,
}

pub struct Shader {
    pipeline: Pipeline,
    device: Rc<Device>,
}

impl Shader {
    pub(crate) fn new(
        device: &Rc<Device>,
        render_pass: &RenderPass,
        vert_source: &[u8],
        frag_source: &[u8],
        frag_mode: FragmentMode,
        cull_mode: CullMode,
        depth: Depth,
        layout: &ShaderLayout,
    ) -> Self {
        let vert_module = create_shader_module(device, vert_source);
        let frag_module = create_shader_module(device, frag_source);
        let entry_point = cstring("main");

        let vs_stage_info = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point)
            .build();

        let fs_stage_info = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point)
            .build();

        let binding_descriptions = [Vertex::binding_description()];
        let attribute_descriptions = Vertex::attribute_descriptions();
        let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
            .build();

        let assembly_input_info = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D {
                width: 1,
                height: 1,
            },
        };

        let viewports = [viewport];
        let scissors = [scissor];
        let viewport_state = PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors)
            .build();

        let mut rasterizer_state = PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(false)
            .front_face(FrontFace::CLOCKWISE)
            .line_width(1.0);

        rasterizer_state = match cull_mode {
            CullMode::Back => rasterizer_state.cull_mode(CullModeFlags::BACK),
            CullMode::Front => rasterizer_state.cull_mode(CullModeFlags::FRONT),
        };

        rasterizer_state = match frag_mode {
            FragmentMode::Fill => rasterizer_state.polygon_mode(PolygonMode::FILL),
            FragmentMode::Lines => rasterizer_state.polygon_mode(PolygonMode::LINE),
        };

        let samples = device.pick_sample_count();

        let multisampling = PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(samples);

        let depth_enabled = match depth {
            Depth::Enabled => true,
            Depth::Disabled => false,
        };

        let stencil = StencilOpState::builder()
            .fail_op(StencilOp::KEEP)
            .pass_op(StencilOp::REPLACE)
            .depth_fail_op(StencilOp::KEEP)
            .compare_op(CompareOp::ALWAYS)
            .compare_mask(1)
            .write_mask(1)
            .reference(1)
            .build();

        let depth_stencil_state = PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(depth_enabled)
            .depth_write_enable(depth_enabled)
            .depth_compare_op(CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(true)
            .front(stencil);

        let color_blend_attachment = PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B
                    | ColorComponentFlags::A,
            )
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .build();

        let attachments = [color_blend_attachment];
        let color_blending = PipelineColorBlendStateCreateInfo::builder()
            .attachments(&attachments)
            .logic_op_enable(false)
            .build();

        let dynamic_states = [
            DynamicState::LINE_WIDTH,
            DynamicState::SCISSOR,
            DynamicState::VIEWPORT,
        ];
        let dynamic_state = PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_states)
            .build();

        let stages = [vs_stage_info, fs_stage_info];
        let pipeline_info = GraphicsPipelineCreateInfo::builder()
            .stages(&stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&assembly_input_info)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer_state)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .depth_stencil_state(&depth_stencil_state)
            .dynamic_state(&dynamic_state)
            .layout(layout.pipeline())
            .render_pass(render_pass.vk())
            .subpass(0)
            .build();

        let pipeline_infos = [pipeline_info];
        let pipeline = unsafe {
            device
                .logical()
                .create_graphics_pipelines(PipelineCache::null(), &pipeline_infos, None)
                .or_error("cannot create pipeline")[0]
        };

        unsafe {
            device.logical().destroy_shader_module(vert_module, None);
            device.logical().destroy_shader_module(frag_module, None);
        }

        Self {
            pipeline,
            device: Rc::clone(device),
        }
    }

    pub(crate) fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_pipeline(self.pipeline, None);
        }
    }
}

fn create_shader_module(device: &Rc<Device>, source: &[u8]) -> ShaderModule {
    let words = read_spv(&mut Cursor::new(&source[..])).or_error("cannot read spv");
    let info = ShaderModuleCreateInfo::builder().code(&words).build();
    unsafe {
        device
            .logical()
            .create_shader_module(&info, None)
            .or_error("cannot create shader module")
    }
}
