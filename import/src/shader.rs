// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// imports glsl shader for use in tegne

use indicatif::ProgressBar;
use regex::Regex;
use shaderc::CompilationArtifact;
use shaderc::CompileOptions;
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

    progress.finish_with_message("done");
    Ok(())
}

fn compile_vert(src: &str) -> Result<CompilationArtifact> {
    let vert_glsl = include_str!("../glsl/vert.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    let define_regex = Regex::new(r"(#define [A-Z]+\s+)*")?;
    let defines = match define_regex.find(src) {
        Some(m) => &src[m.start()..m.end()],
        None => "",
    };

    // create real glsl code
    let real_src = format!(
        "#version 450\n{}\n{}\n{}\n{}\n{}\nvoid main() {{ vertex(); }}",
        objects_glsl, vert_glsl, defines, srgb_glsl, src
    );

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let mut options = CompileOptions::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    options.add_macro_definition("VERTEX", Some("1"));
    let artifact = compiler.compile_into_spirv(
        &real_src,
        ShaderKind::Vertex,
        "shader.vert",
        "main",
        Some(&options),
    )?;
    Ok(artifact)
}

fn compile_frag(src: &str) -> Result<CompilationArtifact> {
    let frag_glsl = include_str!("../glsl/frag.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let phong_glsl = include_str!("../glsl/phong.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    let define_regex = Regex::new(r"(#define [A-Z]+\s+)*")?;
    let defines = match define_regex.find(src) {
        Some(m) => &src[m.start()..m.end()],
        None => "",
    };

    // create real glsl code
    let is_depth_frag = src.find("out_color").is_none();
    let out_color = if is_depth_frag {
        ""
    } else {
        "layout(location = 0) out vec4 out_color;"
    };

    let real_src = format!(
        "#version 450\n{}\n{}\n{}\n{}\n{}\n{}\n{}\nvoid main() {{ fragment(); }}",
        objects_glsl, frag_glsl, out_color, defines, phong_glsl, srgb_glsl, src
    );

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let mut options = CompileOptions::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    options.add_macro_definition("FRAGMENT", Some("1"));
    let artifact = compiler.compile_into_spirv(
        &real_src,
        ShaderKind::Fragment,
        "shader.frag",
        "main",
        Some(&options),
    )?;
    Ok(artifact)
}
