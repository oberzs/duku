// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// image subcommand

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
use import::import_image;

pub fn image(mut args: Vec<String>, help: bool, no_color: bool) {
    if help {
        image_help(no_color);
    } else {
        // gather arguments
        let relative = args
            .iter()
            .position(|a| *a == "--relative" || *a == "-r")
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
            out_dir_path.join(Path::new(&format!("{}.image", name)))
        };
        fs::create_dir_all(&out_dir_path).expect("bad dir");

        // import image
        eprint!(
            "Converting {} ... ",
            in_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        io::stderr().lock().flush().expect("bad flush");

        let image_data = fs::read(&in_path).expect("bad read");
        let binary = match import_image(&image_data) {
            Ok(bin) => bin,
            Err(err) => error(format!("{}", err), no_color),
        };
        let mut out_file = File::create(&out_path).expect("bad file");
        out_file.write_all(&binary).expect("bad write");

        eprintln!("done");
    }
}

fn image_help(no_color: bool) {
    eprintln!(
        r#"imports image file (.png)

{}
    $ dai image [OPTIONS]

{}
    -i, --in [file]         {}
    -o, --out [directory]   {}
    -r, --relative          {}

{}
    $ dai image --relative --in image.png
"#,
        title("USAGE", no_color),
        title("OPTIONS", no_color),
        desc("input image file (.png)", no_color),
        desc("output file directory", no_color),
        desc("use input file directory for output", no_color),
        title("EXAMPLE", no_color)
    );
}
