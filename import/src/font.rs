// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports ttf font for use in draw-it

use image::DynamicImage;
use image::GenericImage;
use rusttype::Font;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::bitmap;
use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;
use crate::sdf::Sdf;

#[derive(Serialize)]
struct FontFile {
    bitmap_fonts: Vec<BitmapFont>,
    sdf_font: SdfFont,
}

#[derive(Serialize)]
struct BitmapFont {
    bitmap_size: u32,
    font_size: u32,
    char_metrics: HashMap<char, CharMetrics>,
    bitmap: Vec<u8>,
}

#[derive(Serialize)]
struct SdfFont {
    bitmap_size: u32,
    font_size: u32,
    margin: u32,
    char_metrics: HashMap<char, CharMetrics>,
    bitmap: Vec<u8>,
}

#[derive(Serialize)]
struct CharMetrics {
    pub x: u32,
    pub y: u32,
    pub advance: u32,
}

pub struct FontOptions<'sizes> {
    pub sdf_sample: u32,
    pub sdf_size: u32,
    pub sdf_margin: u16,
    pub bitmap_sizes: &'sizes [u32],
}

pub fn import_font(in_path: &Path, out_path: &Path, options: FontOptions<'_>) -> Result<()> {
    eprint!(
        "Converting {} ... ",
        in_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    );
    io::stderr().lock().flush()?;

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?!:-_+=@#(){}[]/";
    let tile_count = (chars.len() as f32).sqrt().ceil() as u32;

    let font_data = fs::read(in_path)?;
    let font =
        Font::try_from_bytes(&font_data).ok_or(ErrorType::Internal(ErrorKind::InvalidFont))?;

    // create sdf font
    let sdf = Sdf::new(options.sdf_sample, options.sdf_size, options.sdf_margin);
    let sdf_tile_size = sdf.sdf_size + sdf.sdf_margin as u32 * 2;
    let sdf_bitmap_size = tile_count * sdf_tile_size;

    let mut sdf_font = SdfFont {
        bitmap_size: sdf_bitmap_size,
        font_size: sdf.sdf_size,
        margin: sdf.sdf_margin as u32,
        char_metrics: HashMap::new(),
        bitmap: vec![],
    };

    let mut sdf_bitmap = DynamicImage::new_luma8(sdf_bitmap_size, sdf_bitmap_size).to_luma();

    for (i, c) in chars.chars().enumerate() {
        let (bitmap, advance) = sdf.generate(&font, c)?;

        let x = (i as u32 % tile_count) * sdf_tile_size;
        let y = (i as u32 / tile_count) * sdf_tile_size;

        sdf_font
            .char_metrics
            .insert(c, CharMetrics { x, y, advance });

        sdf_bitmap.copy_from(&bitmap, x, y)?;
    }

    sdf_font.bitmap = sdf_bitmap.into_raw();

    // create bitmap fonts
    let mut bitmap_fonts = Vec::with_capacity(options.bitmap_sizes.len());
    for font_size in options.bitmap_sizes {
        let bitmap_size = tile_count * font_size;
        let mut bitmap = DynamicImage::new_luma8(bitmap_size, bitmap_size).to_luma();
        let mut char_metrics = HashMap::new();

        for (i, c) in chars.chars().enumerate() {
            // ttf to png
            let (char_bitmap, advance) = bitmap::rasterize(&font, *font_size, 0, c)?;

            let x = (i as u32 % tile_count) * *font_size;
            let y = (i as u32 / tile_count) * *font_size;

            char_metrics.insert(
                c,
                CharMetrics {
                    x,
                    y,
                    advance: advance as u32,
                },
            );

            bitmap.copy_from(&char_bitmap, x, y)?;
        }

        bitmap_fonts.push(BitmapFont {
            font_size: *font_size,
            bitmap_size,
            char_metrics,
            bitmap: bitmap.into_raw(),
        });
    }

    // write fonts to file
    let data = FontFile {
        bitmap_fonts,
        sdf_font,
    };

    let binary = bincode::serialize(&data)?;
    let out_path = out_path.with_extension("font");
    let mut out_file = File::create(out_path)?;

    out_file.write_all(&binary)?;

    eprintln!("done");
    Ok(())
}
