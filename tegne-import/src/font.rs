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

use crate::sdf::CharMetrics;
use crate::sdf::SDF;

#[derive(Serialize)]
struct AtlasMetrics {
    sdf_size: u32,
    atlas_size: u32,
    char_count: u32,
    char_metrics: HashMap<char, CharMetrics>,
}

pub fn import_font(in_path: &Path, out_path: &Path) {
    println!("Compiling {:?}", in_path);

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.?!(){}[]/";
    let font_size = 128;
    let font_margin = 10;
    let sdf_size = 32;
    let atlas_size = (chars.len() as f32).sqrt().ceil() as u32;

    let progress = ProgressBar::new(4 + chars.len() as u64);

    let font_data = fs::read(in_path).expect("cannot read input font");
    let font = Font::try_from_bytes(&font_data).expect("cannot construct font");

    let mut atlas = DynamicImage::new_rgba8(atlas_size * sdf_size, atlas_size * sdf_size).to_rgba();
    let mut atlas_metrics = AtlasMetrics {
        sdf_size,
        atlas_size,
        char_count: chars.len() as u32,
        char_metrics: HashMap::new(),
    };

    progress.inc(1);

    for (i, c) in chars.chars().enumerate() {
        let (img, metrics) = SDF::new(&font, c)
            .with_font_size(font_size)
            .with_font_margin(font_margin)
            .with_sdf_size(sdf_size)
            .generate()
            .expect("cannot generate sdf");
        atlas_metrics.char_metrics.insert(c, metrics);

        let x = (i as u32 % atlas_size) * sdf_size;
        let y = (i as u32 / atlas_size) * sdf_size;
        atlas
            .copy_from(&img, x, y)
            .expect("cannot copy sdf to atlas");

        progress.inc(1);
    }
    let img_raw = atlas.into_raw();
    let json = serde_json::to_string_pretty(&atlas_metrics)
        .expect("cannot create json")
        .into_bytes();

    // compress font
    let out_path = out_path.with_extension("font");
    let out_file = File::create(out_path).expect("cannot create file");
    let mut archive = Builder::new(out_file);
    progress.inc(1);

    let mut img_header = Header::new_gnu();
    img_header.set_size(img_raw.len() as u64);
    img_header.set_cksum();
    archive
        .append_data(&mut img_header, "atlas.img", img_raw.as_slice())
        .expect("cannot add to archive");
    progress.inc(1);

    let mut json_header = Header::new_gnu();
    json_header.set_size(json.len() as u64);
    json_header.set_cksum();
    archive
        .append_data(&mut json_header, "atlas.json", json.as_slice())
        .expect("cannot add to archive");
    progress.inc(1);

    progress.finish_with_message("done");
}
