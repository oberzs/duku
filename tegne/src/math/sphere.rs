// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// simple sphere struct

use super::Vector3;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Sphere {
    pub(crate) center: Vector3,
    pub(crate) radius: f32,
}
