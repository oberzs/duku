// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// pipeline is responsible for the whole rendering pass

mod attachment;
mod image_uniform;
mod layout;
mod material;
mod properties;
mod render_pass;
mod sampler;
mod shader;

use properties::CullMode;
use properties::DepthMode;
use properties::ShapeMode;
use sampler::Sampler;
use sampler::SamplerOptions;

pub(crate) use attachment::Attachment;
pub(crate) use attachment::AttachmentOptions;
pub(crate) use image_uniform::ImageUniform;
pub(crate) use layout::Descriptor;
pub(crate) use layout::PushConstants;
pub(crate) use layout::ShaderLayout;
pub(crate) use material::CoreMaterial;
pub(crate) use material::MaterialData;
pub(crate) use render_pass::RenderPass;
pub(crate) use shader::CoreShader;

pub use material::Material;
pub use shader::Shader;
