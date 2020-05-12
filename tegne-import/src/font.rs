use image::DynamicImage;
use image::GenericImage;
use indicatif::ProgressBar;
use rusttype::Font;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
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
    char_metrics: HashMap<char, CharMetrics>,
}

pub fn import_font(in_path: &Path, out_path: &Path) -> Result<()> {
    println!("Converting {:?}", in_path);

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,?!:-_+=@#(){}[]/";
    let options = SdfOptions {
        font_size: 1024,
        font_margin: 50,
        sdf_size: 256,
    };
    let tile_count = (chars.len() as f32).sqrt().ceil() as u32;
    let tile_size = options.sdf_size;
    let atlas_size = tile_count * tile_size;

    let progress = ProgressBar::new(4 + chars.len() as u64);

    let font_data = fs::read(in_path)?;
    let font =
        Font::try_from_bytes(&font_data).ok_or(ErrorType::Internal(ErrorKind::InvalidFont))?;

    let mut atlas_metrics = AtlasMetrics {
        sdf_size: options.sdf_size,
        atlas_size,
        char_metrics: HashMap::new(),
    };

    let mut atlas = DynamicImage::new_rgba8(atlas_size, atlas_size).to_rgba();

    progress.inc(1);

    for (i, c) in chars.chars().enumerate() {
        let mut char_data = generate_sdf(&font, c, options)?;

        let x = (i as u32 % tile_count) * tile_size;
        let y = (i as u32 / tile_count) * tile_size;

        char_data.metrics.x = x;
        char_data.metrics.y = y;

        atlas_metrics.char_metrics.insert(c, char_data.metrics);

        atlas.copy_from(&char_data.image, x, y)?;

        progress.inc(1);
    }

    let img_raw = atlas.into_raw();
    let json = serde_json::to_string_pretty(&atlas_metrics)?.into_bytes();

    // compress font
    let out_path = out_path.with_extension("font");
    let out_file = File::create(out_path)?;
    let mut archive = Builder::new(out_file);
    progress.inc(1);

    let mut img_header = Header::new_gnu();
    img_header.set_size(img_raw.len() as u64);
    img_header.set_cksum();
    archive.append_data(&mut img_header, "atlas.img", img_raw.as_slice())?;
    progress.inc(1);

    let mut json_header = Header::new_gnu();
    json_header.set_size(json.len() as u64);
    json_header.set_cksum();
    archive.append_data(&mut json_header, "atlas.json", json.as_slice())?;
    progress.inc(1);

    progress.finish_with_message("done");
    Ok(())
}
