// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// small utility functions

pub fn clamp(n: u32, min: u32, max: u32) -> u32 {
    if n > max {
        max
    } else if n < min {
        min
    } else {
        n
    }
}

#[cfg(test)]
mod test {
    use super::clamp;

    #[test]
    fn clamp_u32() {
        assert_eq!(clamp(7, 0, 5), 5);
        assert_eq!(clamp(3, 4, 6), 4);
        assert_eq!(clamp(3, 1, 7), 3);
    }
}
