// Oliver Berzs
// https://github.com/oberzs/duku

/// The stats of the current target's render.
///
/// Can be used to diagnose the complexity of your
/// render loop, like resource rebinds and draw calls.
/// Should be called after the drawing has been done.
///
/// # Examples
///
/// ```no_run
/// # use duku::Duku;
/// let (mut duku, _) = Duku::windowed(1, 1).unwrap();
///
/// // after drawing
/// let stats = duku.stats();
/// ```
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Stats {
    /// the number of indices that have been drawn
    pub drawn_indices: u32,
    /// the amount of shaders that have been used
    pub shaders_used: u32,
    /// the amount of times a shader has been bound
    pub shader_rebinds: u32,
    /// the amount of materials that have been used
    pub materials_used: u32,
    /// the amount of times a material has been bound
    pub material_rebinds: u32,
    /// the number of draw calls that have been made
    pub draw_calls: u32,
}
