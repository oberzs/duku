// Oliver Berzs
// https://github.com/oberzs/duku

#![cfg(feature = "png")]

use png_dep::BitDepth;
use png_dep::ColorType;
use png_dep::Decoder;
use png_dep::Encoder;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::duku::Duku;
use crate::error::Error;
use crate::error::Result;
use crate::image::ColorSpace;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Format;
use crate::image::Mips;
use crate::image::Texture;
use crate::storage::Handle;

pub(crate) struct PngData {
    pub(crate) data: Vec<u8>,
    pub(crate) format: Format,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Duku {
    pub fn create_texture_png(
        &mut self,
        path: impl AsRef<Path>,
        color_space: ColorSpace,
        mips: Mips,
    ) -> Result<Handle<Texture>> {
        let bytes = fs::read(path.as_ref())?;
        self.create_texture_png_bytes(&bytes, color_space, mips)
    }

    pub fn create_texture_png_bytes(
        &mut self,
        bytes: &[u8],
        color_space: ColorSpace,
        mips: Mips,
    ) -> Result<Handle<Texture>> {
        let png_data = load_png(bytes, color_space)?;
        self.create_texture(
            png_data.data,
            png_data.format,
            mips,
            png_data.width,
            png_data.height,
        )
    }

    pub fn save_texture(&self, texture: &Handle<Texture>, path: impl AsRef<Path>) -> Result<()> {
        let tex = self.texture(texture);

        let file = File::create(path.as_ref())?;

        let mut encoder = Encoder::new(BufWriter::new(file), tex.width(), tex.height());
        encoder.set_color(ColorType::RGBA);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().expect("bad write");

        writer.write_image_data(tex.data()).expect("bad write");

        Ok(())
    }

    pub fn create_cubemap_png(
        &mut self,
        color_space: ColorSpace,
        sides: CubemapSides<impl AsRef<Path>>,
    ) -> Result<Handle<Cubemap>> {
        self.create_cubemap_png_bytes(
            color_space,
            CubemapSides {
                top: &fs::read(sides.top)?,
                bottom: &fs::read(sides.bottom)?,
                left: &fs::read(sides.left)?,
                right: &fs::read(sides.right)?,
                front: &fs::read(sides.front)?,
                back: &fs::read(sides.back)?,
            },
        )
    }

    pub fn create_cubemap_png_bytes(
        &mut self,
        color_space: ColorSpace,
        sides: CubemapSides<&[u8]>,
    ) -> Result<Handle<Cubemap>> {
        let top = load_png(sides.top, color_space)?;
        let bottom = load_png(sides.bottom, color_space)?;
        let left = load_png(sides.left, color_space)?;
        let right = load_png(sides.right, color_space)?;
        let front = load_png(sides.front, color_space)?;
        let back = load_png(sides.back, color_space)?;

        // validate cubemap sides
        if top.width != top.height {
            return Err(Error::InvalidPng);
        }
        if (&[
            bottom.format,
            left.format,
            right.format,
            front.format,
            back.format,
        ])
            .iter()
            .any(|f| *f != top.format)
        {
            return Err(Error::InvalidPng);
        }
        if (&[
            bottom.width,
            left.width,
            right.width,
            front.width,
            back.width,
        ])
            .iter()
            .any(|w| *w != top.width)
        {
            return Err(Error::InvalidPng);
        }
        if (&[
            bottom.height,
            left.height,
            right.height,
            front.height,
            back.height,
        ])
            .iter()
            .any(|h| *h != top.height)
        {
            return Err(Error::InvalidPng);
        }

        self.create_cubemap(
            top.format,
            top.width,
            CubemapSides {
                top: top.data,
                bottom: bottom.data,
                left: left.data,
                right: right.data,
                front: front.data,
                back: back.data,
            },
        )
    }
}

pub(crate) fn load_png(bytes: &[u8], color_space: ColorSpace) -> Result<PngData> {
    let decoder = Decoder::new(bytes);
    let (info, mut reader) = decoder.read_info().map_err(|_| Error::InvalidPng)?;

    let mut data = vec![0; info.buffer_size()];
    reader
        .next_frame(&mut data)
        .map_err(|_| Error::InvalidPng)?;

    let format = match info.color_type {
        ColorType::RGBA if color_space == ColorSpace::Linear => Format::Rgba,
        ColorType::RGBA => Format::Srgba,
        ColorType::RGB if color_space == ColorSpace::Linear => Format::Rgb,
        ColorType::RGB => Format::Srgb,
        ColorType::Grayscale => Format::Gray,
        _ => return Err(Error::UnsupportedFormat),
    };

    Ok(PngData {
        width: info.width,
        height: info.height,
        format,
        data,
    })
}
