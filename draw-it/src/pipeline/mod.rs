// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// pipeline is responsible for the whole rendering pass

mod attachment;
mod layout;
mod material;
mod properties;
mod render_pass;
mod sampler;
mod shader;
mod uniform;
mod uniform_data;

use properties::CullMode;
use properties::DepthMode;
use properties::ShapeMode;
use sampler::Sampler;
use sampler::SamplerOptions;

pub(crate) use attachment::Attachment;
pub(crate) use attachment::AttachmentOptions;
pub(crate) use layout::ShaderLayout;
pub(crate) use material::CoreMaterial;
pub(crate) use material::MaterialUpdateData;
pub(crate) use render_pass::RenderPass;
pub(crate) use uniform::Descriptor;
pub(crate) use uniform::ImageUniform;
pub(crate) use uniform::ShadowMapUniform;
pub(crate) use uniform::Uniform;
pub(crate) use uniform::WorldUniform;
pub(crate) use uniform_data::LightData;
pub(crate) use uniform_data::PushConstants;
pub(crate) use uniform_data::WorldUniformData;

pub use material::Material;
pub use shader::Shader;
