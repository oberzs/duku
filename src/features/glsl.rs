// Oliver Berzs
// https://github.com/oberzs/duku

//! Optional feature `glsl` module for GLSL file support.

#![cfg(feature = "glsl")]

use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

use super::glsl_compiler::compile;
use crate::duku::Duku;
use crate::error::Result;
use crate::pipeline::Shader;
use crate::resources::Handle;

/// Metadata for shader hot-reloading.
#[derive(Debug)]
pub struct Metadata {
    file: File,
    last_modified: Option<SystemTime>,
}

impl Duku {
    /// Create a shader from a GLSL file
    pub fn create_shader_glsl(&mut self, path: impl AsRef<Path>) -> Result<Handle<Shader>> {
        let source = fs::read_to_string(&path)?;
        self.create_shader_glsl_str(&source)
    }

    /// Create a shader from GLSL source
    pub fn create_shader_glsl_str(&mut self, source: &str) -> Result<Handle<Shader>> {
        let (vert, frag, bytes) = compile(source)?;
        self.create_shader_bytes(&vert, &frag, bytes)
    }
}

impl Metadata {
    /// Initialize metadata for a file
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path.as_ref())?;

        Ok(Self {
            last_modified: None,
            file,
        })
    }

    /// Check if file has been modified
    pub fn is_modified(&mut self) -> bool {
        let metadata = self.file.metadata().expect("bad metadata");
        let modified = metadata.modified().expect("bad modified");

        if let Some(m) = self.last_modified {
            if m != modified {
                self.last_modified = Some(modified);
                return true;
            }
        }

        self.last_modified = Some(modified);
        false
    }
}
