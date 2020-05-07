use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use rusttype::point;
use rusttype::Font;
use rusttype::Scale;
use serde::Serialize;
use spiral::ManhattanIterator;
use std::cmp;

pub struct SDF<'font> {
    font: &'font Font<'font>,
    c: char,
    font_size: u32,
    font_margin: u32,
    sdf_size: u32,
}

#[derive(Serialize)]
pub struct CharMetrics {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offset_x: u32,
    offset_y: u32,
}

impl<'font> SDF<'font> {
    pub fn new(font: &'font Font<'font>, c: char) -> Self {
        Self {
            font,
            c,
            font_size: 64,
            font_margin: 0,
            sdf_size: 32,
        }
    }

    pub fn with_font_size(&mut self, size: u32) -> &mut Self {
        self.font_size = size;
        self
    }

    pub fn with_font_margin(&mut self, margin: u32) -> &mut Self {
        self.font_margin = margin;
        self
    }

    pub fn with_sdf_size(&mut self, size: u32) -> &mut Self {
        self.sdf_size = size;
        self
    }

    pub fn generate(&self) -> Result<(ImageBuffer<Rgba<u8>, Vec<u8>>, CharMetrics), ()> {
        // ttf to png
        let scale = Scale::uniform(self.font_size as f32);
        let glyph = self
            .font
            .glyph(self.c)
            .scaled(scale)
            .positioned(point(self.font_margin as f32, self.font_margin as f32));

        let bounds = glyph.pixel_bounding_box().ok_or(())?;
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;
        let img_size = cmp::max(width, height) + self.font_margin * 2;
        let margin_x = self.font_margin + (img_size - self.font_margin * 2 - width) / 2;
        let margin_y = self.font_margin + (img_size - self.font_margin * 2 - height) / 2;

        let metrics = CharMetrics {
            x: margin_x,
            y: margin_y,
            width,
            height,
            offset_x: 0,
            offset_y: 0,
        };

        let mut img = DynamicImage::new_rgba8(img_size, img_size).to_rgba();
        glyph.draw(|x, y, v| {
            let value = (v * 255.0) as u8;
            let color = Rgba([value, value, value, value]);
            img.put_pixel(x + margin_x, y + margin_y, color);
        });

        // png to sdf
        let sdf_img = ImageBuffer::from_fn(self.sdf_size, self.sdf_size, |x, y| {
            let value = self.distance_to_zone(&img, x, y);
            Rgba([value, value, value, value])
        });

        Ok((sdf_img, metrics))
    }

    fn distance_to_zone(&self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>, out_x: u32, out_y: u32) -> u8 {
        let threshold = 127;
        let max_distance = 512;

        let mid_x = (out_x * img.width()) / self.sdf_size;
        let mid_y = (out_y * img.height()) / self.sdf_size;

        let is_inside = img.get_pixel(mid_x, mid_y)[0] > threshold;

        let mut closest_distance = max_distance as f32;
        for (x, y) in ManhattanIterator::new(mid_x as i32, mid_y as i32, max_distance as u16) {
            if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
                continue;
            }

            let value = img.get_pixel(x as u32, y as u32)[0];
            if (value >= threshold) == is_inside {
                continue;
            }

            let dx = mid_x as i32 - x;
            let dy = mid_y as i32 - y;
            closest_distance = ((dx * dx + dy * dy) as f32).sqrt();
            break;
        }

        // outside = [0.0, 0.5], inside = [0.5, 1.0]
        let distance = match is_inside {
            true => 0.5 + (closest_distance / 2.0) / max_distance as f32,
            false => 0.5 - (closest_distance / 2.0) / max_distance as f32,
        };

        (distance * 255.0) as u8
    }
}
