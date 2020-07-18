// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// RenderStats - rendering metrics

use std::ops::Add;
use std::ops::AddAssign;

#[derive(Copy, Clone, Default)]
pub struct RenderStats {
    pub time: f32,
    pub drawn_indices: u32,
    pub shaders_used: u32,
    pub shader_rebinds: u32,
    pub materials_used: u32,
    pub material_rebinds: u32,
    pub draw_calls: u32,
}

impl RenderStats {
    pub fn drawn_triangles(&self) -> u32 {
        self.drawn_indices / 3
    }
}

impl Add<Self> for RenderStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            time: self.time + rhs.time,
            drawn_indices: self.drawn_indices + rhs.drawn_indices,
            shaders_used: self.shaders_used + rhs.shaders_used,
            shader_rebinds: self.shader_rebinds + rhs.shader_rebinds,
            materials_used: self.materials_used + rhs.materials_used,
            material_rebinds: self.material_rebinds + rhs.material_rebinds,
            draw_calls: self.draw_calls + rhs.draw_calls,
        }
    }
}

impl AddAssign<Self> for RenderStats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
