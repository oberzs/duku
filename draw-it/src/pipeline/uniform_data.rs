// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// uniform data structs for usage in shaders
// must be compatible with /draw-it-import/glsl/objects.glsl

use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct WorldUniformData {
    pub(crate) world_matrix: Matrix4,
    pub(crate) lights: [LightData; 4],
    pub(crate) camera_position: Vector3,
    pub(crate) time: f32,
    pub(crate) light_matrices: [Matrix4; 4],
    pub(crate) cascade_splits: [f32; 4],
    pub(crate) bias: f32,
    pub(crate) pcf: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PushConstants {
    pub(crate) model_matrix: Matrix4,
    pub(crate) albedo_index: i32,
    pub(crate) sampler_index: i32,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub(crate) struct LightData {
    pub(crate) coords: Vector3,
    pub(crate) light_type: i32,
    pub(crate) color: Vector4,
}
