// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports glsl shader for use in draw-it

use serde::Serialize;
use shaderc::CompilationArtifact;
use shaderc::Compiler;
use shaderc::ShaderKind;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

#[derive(Serialize)]
struct ShaderFile {
    vert: Vec<u8>,
    frag: Vec<u8>,
    depth_mode: String,
    shape_mode: String,
    cull_mode: String,
}

#[derive(Debug)]
struct Defines {
    values: HashMap<String, String>,
}

pub fn import_shader(in_path: &Path, out_path: &Path) -> Result<()> {
    eprint!(
        "Converting {} ... ",
        in_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    );
    io::stderr().lock().flush()?;

    let shader_src = fs::read_to_string(in_path)?;
    let defines = Defines::new(&shader_src);

    // get depth mode
    let depth_mode = defines.get("DEPTH");
    if depth_mode.is_empty() {
        return Err(ErrorKind::InvalidShader(
            "depth mode not set. set with '#define DEPTH <mode>'".to_string(),
        )
        .into());
    }
    if !matches!(
        depth_mode.as_str(),
        "test" | "write" | "test_and_write" | "disabled"
    ) {
        return Err(
            ErrorKind::InvalidShader(format!("invalid depth mode value '{}'", depth_mode)).into(),
        );
    }

    // get shape mode
    let shape_mode = defines.get("SHAPE");
    if shape_mode.is_empty() {
        return Err(ErrorKind::InvalidShader(
            "shape mode not set. set with '#define SHAPE <mode>'".to_string(),
        )
        .into());
    }
    if !matches!(
        shape_mode.as_str(),
        "lined_triangles" | "filled_triangles" | "lines"
    ) {
        return Err(
            ErrorKind::InvalidShader(format!("invalid shape mode value '{}'", shape_mode)).into(),
        );
    }

    // get cull mode
    let cull_mode = defines.get("CULL");
    if cull_mode.is_empty() {
        return Err(ErrorKind::InvalidShader(
            "cull mode not set. set with '#define CULL <mode>'".to_string(),
        )
        .into());
    }
    if !matches!(cull_mode.as_str(), "back" | "front" | "disabled") {
        return Err(
            ErrorKind::InvalidShader(format!("invalid cull mode value '{}'", cull_mode)).into(),
        );
    }

    let vert_bin = compile_vert(&defines)?;
    let frag_bin = compile_frag(&shader_src, &defines)?;

    // Create .shader file
    let data = ShaderFile {
        vert: vert_bin.as_binary_u8().to_owned(),
        frag: frag_bin.as_binary_u8().to_owned(),
        depth_mode,
        shape_mode,
        cull_mode,
    };

    let binary = bincode::serialize(&data)?;

    let out_path = out_path.with_extension("shader");
    let mut out_file = File::create(out_path)?;

    out_file.write_all(&binary)?;

    eprintln!("done");
    Ok(())
}

fn compile_vert(defines: &Defines) -> Result<CompilationArtifact> {
    let mut vert_glsl = include_str!("../glsl/vert.glsl").to_string();
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // pick output position format
    let out_position = if defines.exists("VERTEX_POSITION_WORLDSPACE") {
        "worldspace_position"
    } else if defines.exists("VERTEX_POSITION_MODELSPACE") {
        "modelspace_position"
    } else if defines.exists("VERTEX_POSITION_SKYBOXSPACE") {
        "screenspace_position.xyww"
    } else {
        "screenspace_position"
    };
    vert_glsl = vert_glsl.replace("{{out_position}}", out_position);

    // pick output color
    let out_color = if defines.exists("VERTEX_COLOR_SRGB") {
        real_src.push_str(srgb_glsl);
        "srgb_to_linear_color(in_color)"
    } else {
        "in_color"
    };
    vert_glsl = vert_glsl.replace("{{out_color}}", out_color);

    // add source
    real_src.push_str(objects_glsl);
    real_src.push_str(&vert_glsl);

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let artifact =
        compiler.compile_into_spirv(&real_src, ShaderKind::Vertex, "shader.vert", "main", None)?;
    Ok(artifact)
}

fn compile_frag(src: &str, defines: &Defines) -> Result<CompilationArtifact> {
    let frag_glsl = include_str!("../glsl/frag.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");
    let shadow_glsl = include_str!("../glsl/shadow.glsl");
    let srgb_glsl = include_str!("../glsl/srgb.glsl");

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // add base source
    real_src.push_str(objects_glsl);
    real_src.push_str(frag_glsl);

    // add modules
    if defines.exists("SRGB") {
        real_src.push_str(srgb_glsl);
    }
    if defines.exists("SHADOW") {
        real_src.push_str(shadow_glsl);
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
        let mut values = HashMap::new();

        for line in src.lines().map(|l| l.trim_start()) {
            if line.starts_with("#define ") {
                let mut parts = line.split_whitespace().skip(1);
                if let Some(name) = parts.next() {
                    let value = parts.next().unwrap_or_default();
                    values.insert(name.to_string(), value.to_string());
                }
            }
        }

        Self { values }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn get(&self, name: &str) -> String {
        self.values.get(name).unwrap_or(&"".to_string()).clone()
    }
}
