// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// imports glsl shader for use in tegne

use indicatif::ProgressBar;
use shaderc::CompilationArtifact;
use shaderc::Compiler;
use shaderc::ShaderKind;
use std::fs;
use std::fs::File;
use std::path::Path;
use tar::Builder;
use tar::Header;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

#[derive(Debug)]
struct Defines {
    phong: bool,
    shadow: bool,
    srgb: bool,
    vertex_color_srgb: bool,
    vertex_position_worldspace: bool,
    vertex_position_modelspace: bool,
}

pub fn import_shader(in_path: &Path, out_path: &Path) -> Result<()> {
    println!("Converting {:?}", in_path.file_name().unwrap_or_default());

    let progress = ProgressBar::new(6);

    let shader_src = fs::read_to_string(in_path)?;
    progress.inc(1);

    let vert_bin = compile_vert(&shader_src)?;
    progress.inc(1);
    let frag_bin = compile_frag(&shader_src)?;
    progress.inc(1);

    // compress spirv shaders
    let out_path = out_path.with_extension("shader");
    let out_file = File::create(out_path)?;
    let mut archive = Builder::new(out_file);
    progress.inc(1);

    let mut vert_header = Header::new_gnu();
    vert_header.set_size(vert_bin.as_binary_u8().len() as u64);
    vert_header.set_cksum();
    archive.append_data(&mut vert_header, "vert.spv", vert_bin.as_binary_u8())?;
    progress.inc(1);

    let mut frag_header = Header::new_gnu();
    frag_header.set_size(frag_bin.as_binary_u8().len() as u64);
    frag_header.set_cksum();
    archive.append_data(&mut frag_header, "frag.spv", frag_bin.as_binary_u8())?;
    progress.inc(1);

    progress.finish_and_clear();
    Ok(())
}

fn compile_vert(src: &str) -> Result<CompilationArtifact> {
    let vert_glsl = include_str!("../glsl/vert.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    let defines = Defines::new(src);

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // add defines before source
    if defines.vertex_color_srgb {
        real_src.push_str("#define VERTEX_COLOR_SRGB\n");
        real_src.push_str("#define SRGB\n");
    }
    if defines.srgb {
        real_src.push_str("#define SRGB\n");
    }
    if defines.vertex_position_worldspace {
        real_src.push_str("#define VERTEX_POSITION_WORLDSPACE\n");
    }
    if defines.vertex_position_modelspace {
        real_src.push_str("#define VERTEX_POSITION_MODELSPACE\n");
    }

    // add objects
    real_src.push_str(objects_glsl);

    // add modules
    real_src.push_str(srgb_glsl);

    // add vertex source
    real_src.push_str(vert_glsl);

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let artifact =
        compiler.compile_into_spirv(&real_src, ShaderKind::Vertex, "shader.vert", "main", None)?;
    Ok(artifact)
}

fn compile_frag(src: &str) -> Result<CompilationArtifact> {
    let frag_glsl = include_str!("../glsl/frag.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let phong_glsl = include_str!("../glsl/phong.glsl");
    let shadow_glsl = include_str!("../glsl/shadow.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    let defines = Defines::new(src);

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // add defines before source
    if defines.srgb {
        real_src.push_str("#define SRGB\n");
    }
    if defines.phong {
        real_src.push_str("#define PHONG\n");
    }
    if defines.shadow {
        real_src.push_str("#define SHADOW\n");
    }

    // add objects
    real_src.push_str(objects_glsl);

    // add base fragment source
    real_src.push_str(frag_glsl);

    // add modules
    real_src.push_str(srgb_glsl);
    real_src.push_str(phong_glsl);
    real_src.push_str(shadow_glsl);

    // add out_color if needed
    if src.contains("out_color") {
        real_src.push_str("layout(location = 0) out vec4 out_color;\n");
    }

    let pre_line_count = real_src.lines().count() as u32;

    // add fragment source
    real_src.push_str(&format!("{}\nvoid main() {{ fragment(); }}", src));

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let artifact =
        compiler.compile_into_spirv(&real_src, ShaderKind::Fragment, "shader.frag", "main", None);

    match artifact {
        Err(shaderc::Error::CompilationError(_, msg)) => {
            // format shader error
            let mut result = "invalid shader code\n".to_string();
            for error in msg.lines() {
                let parts = error.split(':').map(|p| p.trim()).collect::<Vec<_>>();

                let line = parts[1].parse::<u32>().expect("bad code") - pre_line_count;
                let reason = format!("{}, {}", parts[3], parts[4]);

                result.push_str(&format!("\x1b[93mat line {}\x1b[0m: {}\n", line, reason,));
            }
            Err(ErrorType::Internal(ErrorKind::InvalidShader(result)))
        }
        Err(err) => Err(err.into()),
        Ok(value) => Ok(value),
    }
}

impl Defines {
    pub fn new(src: &str) -> Self {
        Self {
            phong: src.contains("#define PHONG"),
            shadow: src.contains("#define SHADOW"),
            srgb: src.contains("#define SRGB"),
            vertex_color_srgb: src.contains("#define VERTEX_COLOR_SRGB"),
            vertex_position_worldspace: src.contains("#define VERTEX_POSITION_WORLDSPACE"),
            vertex_position_modelspace: src.contains("#define VERTEX_POSITION_MODELSPACE"),
        }
    }
}
