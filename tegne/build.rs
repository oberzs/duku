use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let in_shader_dir = "shaders";
    let out_shader_dir = format!(
        "{}/shaders",
        env::var("OUT_DIR").expect("OUT_DIR not defined")
    );

    fs::create_dir_all(&out_shader_dir).expect("cannot create out directory");

    fs::read_dir(in_shader_dir)
        .expect("cannot read directory")
        .into_iter()
        .map(|entry| entry.expect("cannot get file in directory"))
        .filter_map(|entry| {
            let name = entry
                .file_name()
                .into_string()
                .expect("cannot convert file name to string");
            if name.ends_with(".glsl") {
                None
            } else {
                Some(name)
            }
        })
        .for_each(|name| {
            let output = Command::new("glslc")
                .arg(format!("{}/{}", in_shader_dir, name))
                .arg("-o")
                .arg(format!("{}/{}.spv", out_shader_dir, name))
                .output()
                .expect("cannot run glslc");

            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            assert!(output.status.success(), stderr);
        });
}
