// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

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

use properties::DependencyType;
use properties::SamplerAddress;
use properties::SamplerFilter;
use sampler::Sampler;
use sampler::SamplerOptions;

pub(crate) use attachment::Attachment;
pub(crate) use attachment::AttachmentOptions;
pub(crate) use layout::ShaderLayout;
pub(crate) use render_pass::RenderPass;
pub(crate) use render_pass::RenderPasses;
pub(crate) use uniform::Descriptor;
pub(crate) use uniform::ImageUniform;
pub(crate) use uniform::MaterialUniform;
pub(crate) use uniform::WorldUniform;
pub(crate) use uniform_data::Light;
pub(crate) use uniform_data::MaterialData;
pub(crate) use uniform_data::PushConstants;
pub(crate) use uniform_data::WorldData;

pub use material::Material;
pub use material::MaterialOptions;
pub use shader::Shader;
pub use shader::ShaderOptions;
