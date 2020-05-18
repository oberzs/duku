use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use rusttype::point;
use rusttype::Font;
use rusttype::Scale;
use serde::Serialize;
use spiral::ManhattanIterator;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

#[derive(Copy, Clone)]
pub struct SdfOptions {
    pub font_size: u32,
    pub font_margin: u32,
    pub sdf_size: u32,
    pub sdf_max: u16,
}

pub struct CharData {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub metrics: CharMetrics,
}

#[derive(Serialize)]
pub struct CharMetrics {
    pub x: u32,
    pub y: u32,
    pub advance: u32,
    pub bearing: u32,
}

impl Default for SdfOptions {
    fn default() -> Self {
        Self {
            font_size: 4096,
            font_margin: 0,
            sdf_size: 64,
            sdf_max: 400,
        }
    }
}

impl SdfOptions {
    pub fn scale_to_sdf(&self, value: f32) -> u32 {
        let rescale = self.sdf_size as f32 / (self.font_size + self.font_margin * 2) as f32;
        (value * rescale).round() as u32
    }
}

pub fn generate_sdf(font: &Font<'_>, c: char, options: SdfOptions) -> Result<CharData> {
    // ttf to png
    let scale = Scale::uniform(options.font_size as f32);
    let glyph = font.glyph(c).scaled(scale).positioned(point(
        options.font_margin as f32,
        options.font_margin as f32,
    ));

    let image_size = options.font_size + options.font_margin * 2;
    let bounds = glyph
        .pixel_bounding_box()
        .ok_or(ErrorType::Internal(ErrorKind::NoBounds))?;
    let ascent = font.v_metrics(scale).ascent.round() as i32;
    let h_metrics = glyph.unpositioned().h_metrics();
    let min_x = bounds.min.x as u32;
    let min_y = (bounds.max.y + (ascent - bounds.height())) as u32;

    let metrics = CharMetrics {
        x: 0,
        y: 0,
        advance: options.scale_to_sdf(h_metrics.advance_width),
        bearing: options.scale_to_sdf(h_metrics.left_side_bearing),
    };

    let mut img = DynamicImage::new_rgba8(image_size, image_size).to_rgba();
    glyph.draw(|x, y, v| {
        let value = (v * 255.0) as u8;
        let color = Rgba([value, value, value, value]);
        let xx = x + min_x;
        let yy = y + min_y;
        if xx < image_size && yy < image_size {
            img.put_pixel(xx, yy, color);
        }
    });

    // png to sdf
    let sdf_img = ImageBuffer::from_fn(options.sdf_size, options.sdf_size, |x, y| {
        let value = distance_to_zone(&img, x, y, options);
        Rgba([value, value, value, value])
    });

    let data = CharData {
        image: sdf_img,
        metrics,
    };

    Ok(data)
}

fn distance_to_zone(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    out_x: u32,
    out_y: u32,
    options: SdfOptions,
) -> u8 {
    let threshold = 127;

    let mid_x = (out_x * img.width()) / options.sdf_size;
    let mid_y = (out_y * img.height()) / options.sdf_size;

    let is_inside = img.get_pixel(mid_x, mid_y)[0] > threshold;

    let mut closest_distance = options.sdf_max as f32;
    for (x, y) in ManhattanIterator::new(mid_x as i32, mid_y as i32, options.sdf_max) {
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
    let distance = if is_inside {
        0.5 + (closest_distance / 2.0) / options.sdf_max as f32
    } else {
        0.5 - (closest_distance / 2.0) / options.sdf_max as f32
    };

    (distance * 255.0) as u8
}
