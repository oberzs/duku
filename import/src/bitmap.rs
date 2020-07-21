// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// rasterizes TTF characters

use image::DynamicImage;
use image::GrayImage;
use image::Luma;
use rusttype::point;
use rusttype::Font;
use rusttype::Scale;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

pub fn rasterize(font: &Font<'_>, size: u32, margin: u32, c: char) -> Result<(GrayImage, f32)> {
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

    let mut bitmap = DynamicImage::new_luma8(image_size, image_size).to_luma();
    glyph.draw(|x, y, v| {
        let color = Luma([(v * 255.0) as u8]);
        let xx = x + margin + bearing.round() as u32;
        let yy = y + margin + (ascent - bounds.height()) as u32;
        if xx < image_size && yy < image_size {
            bitmap.put_pixel(xx, yy, color);
        }
    });

    Ok((bitmap, advance))
}
