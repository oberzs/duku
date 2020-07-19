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

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;
use crate::sdf::generate_sdf;
use crate::sdf::CharMetrics;
use crate::sdf::SdfOptions;

#[derive(Serialize)]
struct FontFile {
    sdf_size: u32,
    atlas_size: u32,
    margin: u32,
    char_metrics: HashMap<char, CharMetrics>,
    atlas: Vec<u8>,
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

    let mut data = FontFile {
        sdf_size: options.sdf_size,
        atlas_size,
        margin: options.scale_to_sdf(options.font_margin as f32),
        char_metrics: HashMap::new(),
        atlas: vec![],
    };

    let mut atlas = DynamicImage::new_rgba8(atlas_size, atlas_size).to_rgba();

    for (i, c) in chars.chars().enumerate() {
        let mut char_data = generate_sdf(&font, c, options)?;

        let x = (i as u32 % tile_count) * tile_size;
        let y = (i as u32 / tile_count) * tile_size;

        char_data.metrics.x = x;
        char_data.metrics.y = y;

        data.char_metrics.insert(c, char_data.metrics);

        atlas.copy_from(&char_data.image, x, y)?;
    }

    data.atlas = atlas.into_raw();

    let binary = bincode::serialize(&data)?;
    let out_path = out_path.with_extension("font");
    let mut out_file = File::create(out_path)?;

    out_file.write_all(&binary)?;

    eprintln!("done");
    Ok(())
}
