use crate::utils::OrError;
use image::GenericImageView;
use std::path::Path;

pub fn read_image(path: impl AsRef<Path>) -> (Vec<u8>, u32, u32) {
    let p = path.as_ref();
    let texture = image::open(p).or_error(format!("cannot open image {}", p.display()));
    let (width, height) = texture.dimensions();
    let data = texture.to_rgba().into_raw();
    (data, width, height)
}
