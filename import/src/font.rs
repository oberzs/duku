// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports ttf font for use in draw-it

use image::DynamicImage;
use image::GenericImage;
use image::ImageBuffer;
use image::Rgba;
use rusttype::Font;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use tar::Builder;
use tar::Header;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;
use crate::sdf::generate_sdf;
use crate::sdf::CharMetrics;
use crate::sdf::SdfOptions;

#[derive(Serialize)]
struct AtlasMetrics {
    sdf_size: u32,
    atlas_size: u32,
    margin: u32,
    char_metrics: HashMap<char, CharMetrics>,
}

pub fn import_font(in_path: &Path, out_path: &Path) -> Result<()> {
    eprint!("Converting {:?} ... ", in_path);
    io::stderr().lock().flush()?;

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?!:-_+=@#(){}[]/";
    let options = SdfOptions::default();
    let tile_count = (chars.len() as f32).sqrt().ceil() as u32;
    let tile_size = options.sdf_size;
    let atlas_size = tile_count * tile_size;

    let font_data = fs::read(in_path)?;
    let font =
        Font::try_from_bytes(&font_data).ok_or(ErrorType::Internal(ErrorKind::InvalidFont))?;

    let mut atlas_metrics = AtlasMetrics {
        sdf_size: options.sdf_size,
        atlas_size,
        margin: options.scale_to_sdf(options.font_margin as f32),
        char_metrics: HashMap::new(),
    };

    let mut atlas = DynamicImage::new_rgba8(atlas_size, atlas_size).to_rgba();

    for (i, c) in chars.chars().enumerate() {
        let mut char_data = generate_sdf(&font, c, options)?;

        let x = (i as u32 % tile_count) * tile_size;
        let y = (i as u32 / tile_count) * tile_size;

        char_data.metrics.x = x;
        char_data.metrics.y = y;
        // let advance = char_data.metrics.advance;
        // let bearing = char_data.metrics.bearing;

        atlas_metrics.char_metrics.insert(c, char_data.metrics);

        atlas.copy_from(&char_data.image, x, y)?;
        // draw_rect(
        //     &mut atlas,
        //     Rgba([255, 0, 0, 255]),
        //     x,
        //     y,
        //     x + tile_size,
        //     y + tile_size,
        // );
        // draw_rect(
        //     &mut atlas,
        //     Rgba([0, 255, 0, 255]),
        //     x,
        //     y + 31,
        //     x + advance,
        //     y + 32,
        // );
        // draw_rect(
        //     &mut atlas,
        //     Rgba([0, 0, 255, 255]),
        //     x,
        //     y + 32,
        //     x + bearing,
        //     y + 33,
        // );
    }

    // atlas.save("test.png")?;

    let img_raw = atlas.into_raw();
    let json = serde_json::to_string_pretty(&atlas_metrics)?.into_bytes();

    // compress font
    let out_path = out_path.with_extension("font");
    let out_file = File::create(out_path)?;
    let mut archive = Builder::new(out_file);

    let mut img_header = Header::new_gnu();
    img_header.set_size(img_raw.len() as u64);
    img_header.set_cksum();
    archive.append_data(&mut img_header, "atlas.img", img_raw.as_slice())?;

    let mut json_header = Header::new_gnu();
    json_header.set_size(json.len() as u64);
    json_header.set_cksum();
    archive.append_data(&mut json_header, "atlas.json", json.as_slice())?;

    eprintln!("done");
    Ok(())
}

fn _draw_rect(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
) {
    for x in x0..x1 {
        img.put_pixel(x, y0, color);
        img.put_pixel(x, y1 - 1, color);
    }
    for y in y0..y1 {
        img.put_pixel(x0, y, color);
        img.put_pixel(x1 - 1, y, color);
    }
}
