// Oliver Berzs
// https://github.com/oberzs/duku

use super::Mix;

/// Color gradient.
///
/// Mixes colors together linearly.
///
/// # Examples
///
/// ```
/// # use duku::Rgb;
/// # use duku::Gradient;
/// let gradient = Gradient::new(vec![
///     Rgb::red(255),
///     Rgb::green(255),
///     Rgb::blue(255),
/// ]);
/// let c1 = gradient.get(0.33);
/// let c2 = gradient.get(0.66);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Gradient<C> {
    values: Vec<C>,
    part: f32,
}

impl<C: Mix + Default + Copy> Gradient<C> {
    /// Create gradient from colors
    ///
    /// If there are less than 2 colors, will extend
    /// with defaults.
    pub fn new(mut values: Vec<C>) -> Self {
        if values.len() == 1 {
            values.push(C::default());
        }

        if values.is_empty() {
            values = vec![C::default(), C::default()];
        }

        let part = 1.0 / values.len() as f32;
        Self { values, part }
    }

    /// Get color at point in gradient
    ///
    /// If `p` is not in range 0 to 1, then
    /// returns default color.
    pub fn get(&self, p: f32) -> C {
        if p < 0.0 || p > 1.0 {
            C::default()
        } else {
            let i = (p / self.part).floor();
            let ip = (p - (self.part * i)) * self.values.len() as f32;
            C::mix(self.values[i as usize], self.values[i as usize + 1], ip)
        }
    }
}
