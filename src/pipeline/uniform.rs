// Oliver Berzs
// https://github.com/oberzs/draw-it

// structs that match shader uniform structs

use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderWorld {
    pub(crate) world_to_view: Matrix4,
    pub(crate) view_to_clip: Matrix4,
    pub(crate) lights: [ShaderLight; 4],
    pub(crate) camera_position: Vector3,
    pub(crate) time: f32,
    pub(crate) world_to_shadow: [Matrix4; 4],
    pub(crate) shadow_cascades: [f32; 4],
    pub(crate) ambient_color: Vector3,
    pub(crate) shadow_bias: f32,
    pub(crate) shadow_pcf: f32,
    pub(crate) skybox_index: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderMaterial {
    pub(crate) arg_1: Vector4,
    pub(crate) arg_2: Vector4,
    pub(crate) arg_3: Vector4,
    pub(crate) arg_4: Vector4,
    pub(crate) arg_5: Vector4,
    pub(crate) arg_6: Vector4,
    pub(crate) arg_7: Vector4,
    pub(crate) arg_8: Vector4,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderLight {
    pub(crate) coords: Vector3,
    pub(crate) light_type: i32,
    pub(crate) color: Vector4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderConstants {
    pub(crate) local_to_world: Matrix4,
    pub(crate) sampler_index: u32,
}
