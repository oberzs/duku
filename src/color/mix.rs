// Oliver Berzs
// https://github.com/oberzs/duku

/// Trait to get color values in-between two.
pub trait Mix {
    /// Calculate color between `from` and `to`
    /// at point `p` in range 0 to 1
    fn mix(from: Self, to: Self, p: f32) -> Self;
}

pub(crate) fn mixf(from: f32, to: f32, p: f32) -> f32 {
    from + (to - from) * p
}

#[cfg(test)]
mod tests {
    use super::mixf;

    #[test]
    fn simple_mix() {
        assert_eq_delta!(mixf(0.0, 1.0, 0.5), 0.5);
        assert_eq_delta!(mixf(1.0, 1.0, 0.5), 1.0);
        assert_eq_delta!(mixf(0.0, 0.0, 0.5), 0.0);
    }
}
