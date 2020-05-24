// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// buffers are used to pass data to the GPU

mod dynamic;
mod fixed;
mod memory;
mod properties;

pub(crate) use dynamic::DynamicBuffer;
pub(crate) use memory::BufferMemory;
pub(crate) use properties::BufferAccess;
pub(crate) use properties::BufferUsage;
