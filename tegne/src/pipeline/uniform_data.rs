// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// uniform data structs for usage in shaders
// must be compatible with /tegne-import/glsl/objects.glsl

use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct WorldData {
    pub(crate) world_matrix: Matrix4,
    pub(crate) light_matrix: Matrix4,
    pub(crate) lights: [Light; 4],
    pub(crate) camera_position: Vector3,
    pub(crate) time: f32,
    pub(crate) cascade_splits: Vector3,
    pub(crate) shadow_index: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct MaterialData {
    pub(crate) albedo_tint: Vector3,
    pub(crate) font_width: f32,
    pub(crate) font_border_tint: Vector3,
    pub(crate) font_edge: f32,
    pub(crate) font_border_offset: Vector2,
    pub(crate) font_border_width: f32,
    pub(crate) font_border_edge: f32,
    pub(crate) arg_1: Vector4,
    pub(crate) arg_2: Vector4,
    pub(crate) arg_3: Vector4,
    pub(crate) arg_4: Vector4,
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
pub(crate) struct Light {
    pub(crate) coords: Vector4,
    pub(crate) color: Vector4,
}
