// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// shader subcommand

mod import;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use super::desc;
use super::error;
use super::title;
use super::warn;
use super::watch::watch_file;
use import::import_shader;

pub fn shader(mut args: Vec<String>, help: bool, no_color: bool) {
    if help {
        shader_help(no_color);
    } else {
        // gather arguments
        let relative = args
            .iter()
            .position(|a| *a == "--relative" || *a == "-r")
            .map(|pos| args.remove(pos))
            .is_some();
        let watch = args
            .iter()
            .position(|a| *a == "--watch" || *a == "-w")
            .map(|pos| args.remove(pos))
            .is_some();

        let mut input = None;
        let mut output = None;
        let mut iter_args = args.into_iter();
        while let Some(arg) = iter_args.next() {
            match arg.as_str() {
                "--in" | "-i" => input = iter_args.next(),
                "--out" | "-o" => output = iter_args.next(),
                _ => error(format!("unknown argument '{}'", arg), no_color),
            }
        }

        // check arguments
        if input.is_none() || (output.is_none() && !relative) {
            error("input and output options not provided", no_color);
        }

        // build in path
        let in_path = PathBuf::from(input.expect("bad input"));

        // check in path
        if !in_path.is_file() {
            error(format!("'{}' is not a file", in_path.display()), no_color);
        }

        // build out path
        let out_dir_path = if relative {
            in_path
                .parent()
                .unwrap_or_else(|| Path::new("./"))
                .to_owned()
        } else {
            PathBuf::from(output.expect("bad output"))
        };
        let out_path = {
            let name = in_path
                .file_stem()
                .expect("bad stem")
                .to_str()
                .expect("bad str");
            out_dir_path.join(Path::new(&format!("{}.shader", name)))
        };
        fs::create_dir_all(&out_dir_path).expect("bad dir");

        // import shader
        eprint!(
            "Converting {} ... ",
            in_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        io::stderr().lock().flush().expect("bad flush");

        let shader_src = fs::read_to_string(&in_path).expect("bad read");
        let binary = match import_shader(&shader_src) {
            Ok(bin) => bin,
            Err(err) => error(format!("{}", err), no_color),
        };
        let mut out_file = File::create(&out_path).expect("bad file");
        out_file.write_all(&binary).expect("bad write");

        eprintln!("done");

        // watch for file changes
        if watch {
            let receiver = watch_file(&in_path);
            while receiver.recv().is_ok() {
                eprint!(
                    "Converting {} ... ",
                    in_path
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                );
                io::stderr().lock().flush().expect("bad flush");

                let shader_src = fs::read_to_string(&in_path).expect("bad read");
                match import_shader(&shader_src) {
                    Err(err) => warn(format!("{}", err), no_color),
                    Ok(bin) => {
                        let mut out_file = File::create(&out_path).expect("bad file");
                        out_file.write_all(&bin).expect("bad write");
                    }
                };

                eprintln!("done");
            }
        }
    }
}

fn shader_help(no_color: bool) {
    eprintln!(
        r#"imports shader file (.glsl)

{}
    $ dai shader [OPTIONS]

{}
    -i, --in [file]         {}
    -o, --out [directory]   {}
    -r, --relative          {}
    -w, --watch             {}

{}
    $ dai shader --relative --in phong.glsl
"#,
        title("USAGE", no_color),
        title("OPTIONS", no_color),
        desc("input shader file (.glsl)", no_color),
        desc("output file directory", no_color),
        desc("use input file directory for output", no_color),
        desc("watch for file changes and recompile", no_color),
        title("EXAMPLE", no_color)
    );
}
