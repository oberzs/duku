// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports png image for use in draw-it

use png::ColorType;
use png::Decoder;
use serde::Serialize;

use crate::error::ErrorKind;
use crate::error::Result;

#[derive(Serialize)]
struct ImageFile {
    data: Vec<u8>,
    width: u32,
    height: u32,
    channels: u8,
}

pub fn import_image(in_data: &[u8]) -> Result<Vec<u8>> {
    let decoder = Decoder::new(in_data);
    let (info, mut reader) = decoder.read_info()?;

    let mut data = vec![0; info.buffer_size()];
    reader.next_frame(&mut data)?;

    // convert data to use 1, 2 or 4 channels
    let channels = match info.color_type {
        ColorType::RGBA => 4,
        ColorType::GrayscaleAlpha => 2,
        ColorType::Grayscale => 1,
        ColorType::Indexed => return Err(ErrorKind::IndexedFormatNotSupported.into()),
        ColorType::RGB => {
            let mut new_data = Vec::with_capacity((data.len() / 3) * 4);
            for pixel in data.chunks(3) {
                new_data.extend(pixel);
                new_data.push(255);
            }
            data = new_data;
            4
        }
    };

    // convert to binary
    let image_file = ImageFile {
        width: info.width,
        height: info.height,
        channels,
        data,
    };

    let binary = bincode::serialize(&image_file)?;
    Ok(binary)
}
