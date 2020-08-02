// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// RenderStats - rendering metrics

#[derive(Copy, Clone, Default)]
pub struct Stats {
    pub drawn_indices: u32,
    pub shaders_used: u32,
    pub shader_rebinds: u32,
    pub materials_used: u32,
    pub material_rebinds: u32,
    pub draw_calls: u32,
    pub time: f32,
    pub delta_time: f32,
    pub fps: u32,
}

impl Stats {
    pub fn drawn_triangles(&self) -> u32 {
        self.drawn_indices / 3
    }
}
