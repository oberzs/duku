// Oliver Berzs
// https://github.com/oberzs/duku

#![cfg(feature = "jpeg")]

use jpeg_dep::Decoder;
use jpeg_dep::PixelFormat;
use std::fs;
use std::path::Path;

use crate::duku::Duku;
use crate::error::Error;
use crate::error::Result;
use crate::image::ColorSpace;
use crate::image::Format;
use crate::image::Mips;
use crate::image::Texture;
use crate::resources::Handle;

pub(crate) struct JpegData {
    pub(crate) data: Vec<u8>,
    pub(crate) format: Format,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Duku {
    /// Create a texture from a JPEG file
    ///
    /// If `options` is `None`, then
    /// sRGB and no mipmaps are used.
    pub fn create_texture_jpeg(
        &mut self,
        path: impl AsRef<Path>,
        options: Option<(ColorSpace, Mips)>,
    ) -> Result<Handle<Texture>> {
        let bytes = fs::read(path.as_ref())?;
        self.create_texture_jpeg_bytes(&bytes, options)
    }

    /// Create a texture from JPEG bytes
    ///
    /// If `options` is `None`, then
    /// sRGB and no mipmaps are used.
    pub fn create_texture_jpeg_bytes(
        &mut self,
        bytes: &[u8],
        options: Option<(ColorSpace, Mips)>,
    ) -> Result<Handle<Texture>> {
        let (color_space, mips) = options.unwrap_or((ColorSpace::Srgb, Mips::Zero));
        let jpeg_data = load_jpeg(bytes, color_space)?;
        self.create_texture(
            jpeg_data.data,
            jpeg_data.format,
            mips,
            jpeg_data.width,
            jpeg_data.height,
        )
    }
}

fn load_jpeg(bytes: &[u8], color_space: ColorSpace) -> Result<JpegData> {
    let mut decoder = Decoder::new(bytes);
    let data = decoder.decode().map_err(|_| Error::InvalidJpeg)?;
    let info = decoder.info().ok_or(Error::InvalidJpeg)?;

    let format = match info.pixel_format {
        PixelFormat::RGB24 if color_space == ColorSpace::Linear => Format::Rgb,
        PixelFormat::RGB24 => Format::Srgb,
        _ => return Err(Error::UnsupportedFormat),
    };

    Ok(JpegData {
        width: u32::from(info.width),
        height: u32::from(info.height),
        format,
        data,
    })
}
