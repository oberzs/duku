// Oliver Berzs
// https://github.com/oberzs/duku

#![cfg(feature = "glsl")]

use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

use super::glsl_compiler::compile;
use crate::duku::Duku;
use crate::error::Result;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConfig;
use crate::storage::Handle;

pub struct Metadata {
    file: File,
    last_modified: Option<SystemTime>,
}

impl Duku {
    pub fn create_shader_glsl(&mut self, path: impl AsRef<Path>) -> Result<Handle<Shader>> {
        let source = fs::read_to_string(&path)?;
        self.create_shader_glsl_str(&source)
    }

    pub fn create_shader_glsl_str(&mut self, source: &str) -> Result<Handle<Shader>> {
        let (vert, frag, bytes) = compile(source)?;
        let config = ShaderConfig {
            depth: bytes[0].try_into()?,
            shape: bytes[1].try_into()?,
            cull: bytes[2].try_into()?,
            outputs: bytes[3],
            msaa: self.msaa(),
        };
        let shader = Shader::new(self.device(), self.uniforms(), &vert, &frag, config)?;

        Ok(self.storage_mut().add_shader(shader))
    }
}

impl Metadata {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path.as_ref())?;

        Ok(Self {
            last_modified: None,
            file,
        })
    }

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
