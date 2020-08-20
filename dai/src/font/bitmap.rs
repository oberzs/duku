// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// rasterizes TTF characters

use rusttype::point;
use rusttype::Font;
use rusttype::Scale;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

pub struct Bitmap {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, value: u8) {
        if x < self.width && y < self.height {
            self.buffer[(x + y * self.width) as usize] = value;
        }
    }

    pub fn copy_from(&mut self, other: &Bitmap, x: u32, y: u32) {
        debug_assert!(self.width >= other.width, "source bitmap is too wide");
        debug_assert!(self.height >= other.height, "source bitmap is too high");
        debug_assert!(
            self.width >= x + other.width && self.height >= y + other.height,
            "source bitmap is positioned ot of bounds"
        );

        // copy row by row
        for row in 0..other.height {
            // prepare destination
            let dst = {
                let begin = (x + (y + row) * self.width) as usize;
                let end = begin + other.width as usize;
                &mut self.buffer[begin..end]
            };

            // prepare source
            let src = {
                let begin = (row * other.width) as usize;
                let end = begin + other.width as usize;
                &other.buffer[begin..end]
            };

            dst.copy_from_slice(src);
        }
    }

    pub fn rasterize(font: &Font<'_>, size: u32, margin: u32, c: char) -> Result<(Self, f32)> {
        let scale = Scale::uniform(size as f32);
        let glyph = font.glyph(c).scaled(scale).positioned(point(0.0, 0.0));

        let image_size = size + margin * 2;
        let bounds = glyph
            .pixel_bounding_box()
            .ok_or(ErrorType::Internal(ErrorKind::NoBounds))?;
        let ascent = font.v_metrics(scale).ascent.round() as i32;
        let h_metrics = glyph.unpositioned().h_metrics();
        let advance = h_metrics.advance_width;
        let bearing = h_metrics.left_side_bearing;

        let mut bitmap = Self::new(image_size, image_size);
        glyph.draw(|x, y, v| {
            let value = (v * 255.0) as u8;
            let xx = x + margin + bearing.round() as u32;
            let yy = y + margin + (ascent - bounds.height()) as u32;
            bitmap.put_pixel(xx, yy, value);
        });

        Ok((bitmap, advance))
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        debug_assert!(
            x < self.width && y < self.height,
            "pixel position out of bounds"
        );
        self.buffer[(x + y * self.width) as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;

    #[test]
    fn new_buffer() {
        let bitmap = Bitmap::new(3, 3);
        assert_eq!(bitmap.buffer.len(), 9);
        assert_eq!(bitmap.width, 3);
        assert_eq!(bitmap.height, 3);
        assert_eq!(bitmap.buffer, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn put_pixel() {
        let mut bitmap = Bitmap::new(3, 3);
        bitmap.put_pixel(1, 1, 1);
        assert_eq!(bitmap.buffer, vec![0, 0, 0, 0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn copy_from() {
        let mut src = Bitmap::new(2, 2);
        src.put_pixel(0, 0, 1);
        src.put_pixel(0, 1, 1);
        src.put_pixel(1, 0, 1);
        src.put_pixel(1, 1, 1);
        let mut dst = Bitmap::new(5, 5);
        dst.copy_from(&src, 2, 1);
        assert_eq!(
            dst.buffer,
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
