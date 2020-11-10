// Oliver Berzs
// https://github.com/oberzs/duku

// pipeline is responsible for the whole rendering pass

mod attachment;
mod material;
mod properties;
mod render_pass;
mod sampler;
mod shader;
mod uniforms;

#[cfg(feature = "glsl")]
mod glsl;

use properties::Clear;
use properties::CullMode;
use properties::DepthMode;
use properties::ShapeMode;
use properties::Store;

pub(crate) use attachment::Attachment;
pub(crate) use render_pass::RenderPass;
pub(crate) use sampler::Sampler;
pub(crate) use shader::ShaderConfig;
pub(crate) use uniforms::Descriptor;
pub(crate) use uniforms::ShaderConstants;
pub(crate) use uniforms::ShaderLight;
pub(crate) use uniforms::ShaderMaterial;
pub(crate) use uniforms::ShaderWorld;
pub(crate) use uniforms::Uniforms;

pub use material::Material;
pub use material::MaterialBuilder;
pub use shader::Shader;
