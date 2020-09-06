// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// pipeline is responsible for the whole rendering pass

mod attachment;
mod images;
mod layout;
mod material;
mod properties;
mod render_pass;
mod sampler;
mod shader;
mod uniform;

use properties::Clear;
use properties::CullMode;
use properties::DepthMode;
use properties::ShapeMode;
use properties::Store;

pub(crate) use attachment::Attachment;
pub(crate) use images::ShaderImages;
pub(crate) use layout::Descriptor;
pub(crate) use layout::ShaderLayout;
pub(crate) use material::CoreMaterial;
pub(crate) use render_pass::RenderPass;
pub(crate) use sampler::Sampler;
pub(crate) use shader::CoreShader;
pub(crate) use uniform::ShaderConstants;
pub(crate) use uniform::ShaderLight;
pub(crate) use uniform::ShaderMaterial;
pub(crate) use uniform::ShaderWorld;

pub use material::Material;
pub use material::MaterialBuilder;
pub use shader::Shader;
