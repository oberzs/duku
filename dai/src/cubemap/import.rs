// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports png cubemap for use in draw-it

use png::ColorType;
use png::Decoder;
use serde::Serialize;

use crate::error::ErrorKind;
use crate::error::Result;

#[derive(Serialize)]
struct CubemapFile {
    top: Vec<u8>,
    bottom: Vec<u8>,
    front: Vec<u8>,
    back: Vec<u8>,
    left: Vec<u8>,
    right: Vec<u8>,
    width: u32,
    height: u32,
    channels: u8,
}

pub fn import_cubemap(in_data: &[Vec<u8>]) -> Result<Vec<u8>> {
    let mut width = None;
    let mut height = None;
    let mut channels = None;
    let mut out_data = vec![];

    for d in in_data {
        let decoder = Decoder::new(d.as_slice());
        let (info, mut reader) = decoder.read_info()?;

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)?;

        // convert data to use 1, 2 or 4 channels
        let c = match info.color_type {
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

        // check if cubemap images match
        if width.is_none() {
            width = Some(info.width);
        } else if width != Some(info.width) {
            return Err(ErrorKind::NonMatchingImages.into());
        }

        if height.is_none() {
            height = Some(info.height);
        } else if height != Some(info.height) {
            return Err(ErrorKind::NonMatchingImages.into());
        }

        if channels.is_none() {
            channels = Some(c);
        } else if channels != Some(c) {
            return Err(ErrorKind::NonMatchingImages.into());
        }

        out_data.push(data);
    }

    // convert to binary
    let image_file = CubemapFile {
        width: width.expect("bad width"),
        height: height.expect("bad height"),
        channels: channels.expect("bad channels"),
        top: out_data.remove(0),
        bottom: out_data.remove(0),
        front: out_data.remove(0),
        back: out_data.remove(0),
        left: out_data.remove(0),
        right: out_data.remove(0),
    };

    let binary = bincode::serialize(&image_file)?;
    Ok(binary)
}
