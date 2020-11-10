// Oliver Berzs
// https://github.com/oberzs/duku

// CLI utility to compile a glsl shader to spirv

#![warn(
    rust_2018_idioms,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    clippy::missing_const_for_fn,
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::if_not_else,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::unused_self,
)]
#![allow(dead_code)]

mod error;

#[path = "pipeline/glsl.rs"]
mod glsl;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use glsl::compile_glsl;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    // show help
    if args.is_empty() || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        show_help(false);
        return;
    }

    // show version
    if args.contains(&"--version".to_string()) {
        println!("{}", VERSION);
        return;
    }

    // check arguments
    let mut input = None;
    let mut output = None;
    let mut no_color = false;
    let mut iter_args = args.into_iter();
    while let Some(arg) = iter_args.next() {
        match arg.as_str() {
            "--out" | "-o" => output = iter_args.next(),
            "--no-color" => no_color = true,
            a => input = Some(a.to_string()),
        }
    }

    // build in path
    let in_path = match input {
        Some(p) => PathBuf::from(p),
        None => error("input file not specified", no_color),
    };

    // check in path
    if !in_path.is_file() {
        error(format!("'{}' is not a file", in_path.display()), no_color);
    }

    // build out path
    let out_dir_path = match output {
        Some(p) => PathBuf::from(p),
        None => in_path
            .parent()
            .unwrap_or_else(|| Path::new("./"))
            .to_owned(),
    };
    let out_path = {
        let name = in_path
            .file_stem()
            .expect("bad stem")
            .to_str()
            .expect("bad str");
        out_dir_path.join(Path::new(&format!("{}.spirv", name)))
    };
    fs::create_dir_all(&out_dir_path).expect("bad dir");

    // compile shader
    let shader_src = fs::read_to_string(&in_path).expect("bad read");
    let (vert, frag, modes) = match compile_glsl(&shader_src) {
        Ok(bin) => bin,
        Err(err) => error(format!("{}", err), no_color),
    };
    let mut binary = vec![];
    binary.extend(&encode_u32(0x5a45ffff));
    binary.push(modes[0]);
    binary.push(modes[1]);
    binary.push(modes[2]);
    binary.extend(&encode_u32(vert.len() as u32));
    binary.extend(&encode_u32(frag.len() as u32));
    binary.extend(&vert);
    binary.extend(&frag);

    let mut out_file = File::create(&out_path).expect("bad file");
    out_file.write_all(&binary).expect("bad write");
}

fn show_help(no_color: bool) {
    eprintln!(
        r#"Duku Shader compiler
    
{}
    {}

{}
    $ dic [FILE]

{}
    -o, --out   {}
    -h, --help  {}
    --no-color  {}
    --version   {}
    "#,
        title("VERSION", no_color),
        VERSION,
        title("USAGE", no_color),
        title("OPTIONS", no_color),
        desc("specifies output directory", no_color),
        desc("shows this help", no_color),
        desc("disables color for output", no_color),
        desc("shows version", no_color)
    );
}

fn error(s: impl AsRef<str>, no_color: bool) -> ! {
    if no_color {
        eprintln!("error: {}", s.as_ref());
    } else {
        eprintln!("\x1b[91merror\x1b[0m: {}", s.as_ref());
    }
    process::exit(1);
}

fn title(s: &str, no_color: bool) -> String {
    if no_color {
        s.to_string()
    } else {
        format!("\x1B[97m{}\x1B[0m", s)
    }
}

fn desc(s: &str, no_color: bool) -> String {
    if no_color {
        s.to_string()
    } else {
        format!("\x1B[90m{}\x1B[0m", s)
    }
}

const fn encode_u32(n: u32) -> [u8; 4] {
    let mut data = [0; 4];
    data[0] = (n >> 24) as u8;
    data[1] = (n >> 16) as u8;
    data[2] = (n >> 8) as u8;
    data[3] = n as u8;
    data
}
