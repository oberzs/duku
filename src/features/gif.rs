// Oliver Berzs
// https://github.com/oberzs/duku

//! Optional feature `gif` module for GIF encoding support.

#![cfg(feature = "gif")]

use gif_dep::Encoder;
use gif_dep::Frame;
use std::fs::File;
use std::path::Path;

use crate::duku::Duku;
use crate::error::Error;
use crate::error::Result;
use crate::image::Canvas;
use crate::resources::Handle;

/// Wrapper around RGB frames used for GIF encoding.
#[derive(Default)]
pub struct Gif {
    frames: Vec<Vec<u8>>,
    size: Option<(u32, u32)>,
}

impl Gif {
    pub(crate) fn encode_frame(&mut self, rgba: Vec<u8>, width: u32, height: u32) -> Result<()> {
        // check size
        if let Some((w, h)) = self.size {
            if w != width || h != height {
                return Err(Error::NonMatchingCanvas);
            }
        } else {
            self.size = Some((width, height));
        }

        let mut rgb = Vec::with_capacity((rgba.len() / 4) * 3);
        for pxs in rgba.chunks(4) {
            rgb.extend(&[pxs[0], pxs[1], pxs[2]]);
        }

        self.frames.push(rgb);

        Ok(())
    }

    /// Save GIF to a file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        if let Some((width, height)) = self.size {
            let mut file = File::create(path)?;
            let mut encoder =
                Encoder::new(&mut file, width as u16, height as u16, &[]).expect("bad gif encoder");

            for frame in &self.frames {
                let f = Frame::from_rgb(width as u16, height as u16, &frame);
                encoder.write_frame(&f).expect("bad gif write");
            }
        }

        Ok(())
    }
}

impl Duku {
    /// Encode the canvas' data into the GIF
    pub fn encode_canvas(&self, gif: &mut Gif, canvas: &Handle<Canvas>) -> Result<()> {
        let (width, height) = {
            let c = canvas.read();
            (c.width, c.height)
        };
        let rgba = self.export_canvas(canvas);
        gif.encode_frame(rgba, width, height)
    }

    /// Encode the window canvas' data into the GIF
    pub fn encode_window_canvas(&self, gif: &mut Gif) -> Result<()> {
        let (width, height) = {
            let c = self.window_canvas();
            (c.width, c.height)
        };
        let rgba = self.export_window_canvas();
        gif.encode_frame(rgba, width, height)
    }
}
