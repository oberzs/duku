// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// RenderStats - rendering metrics

use std::ops::Add;
use std::ops::AddAssign;

#[derive(Copy, Clone, Default)]
pub struct RenderStats {
    pub time: f32,
    pub drawn_indices: u32,
    pub drawn_triangles: u32,
    pub shaders_used: u32,
    pub materials_used: u32,
    pub draw_calls: u32,
}

impl Add<Self> for RenderStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            time: self.time + rhs.time,
            drawn_indices: self.drawn_indices + rhs.drawn_indices,
            drawn_triangles: self.drawn_triangles + rhs.drawn_triangles,
            shaders_used: self.shaders_used + rhs.shaders_used,
            materials_used: self.materials_used + rhs.materials_used,
            draw_calls: self.draw_calls + rhs.draw_calls,
        }
    }
}

impl AddAssign<Self> for RenderStats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
