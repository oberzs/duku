// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// image reading from file

use image_file::error::ImageResult;
use image_file::GenericImageView;
use std::path::Path;

pub fn read_image(path: impl AsRef<Path>) -> ImageResult<(Vec<u8>, u32, u32)> {
    let image = image_file::open(path.as_ref())?;
    let (width, height) = image.dimensions();
    let data = image.to_rgba().into_raw();
    Ok((data, width, height))
}
