// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// cubemap subcommand

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
use import::import_cubemap;

pub fn cubemap(mut args: Vec<String>, help: bool, no_color: bool) {
    if help {
        cubemap_help(no_color);
    } else {
        // gather arguments
        let relative = args
            .iter()
            .position(|a| *a == "--relative" || *a == "-r")
            .map(|pos| args.remove(pos))
            .is_some();

        let mut inputs = vec![];
        let mut output = None;
        let mut iter_args = args.into_iter();
        while let Some(arg) = iter_args.next() {
            match arg.as_str() {
                "--in" | "-i" => {
                    inputs.push(iter_args.next());
                    inputs.push(iter_args.next());
                    inputs.push(iter_args.next());
                    inputs.push(iter_args.next());
                    inputs.push(iter_args.next());
                    inputs.push(iter_args.next());
                }
                "--out" | "-o" => output = iter_args.next(),
                _ => error(format!("unknown argument '{}'", arg), no_color),
            }
        }

        // check arguments
        if inputs.iter().any(|i| i.is_none()) || (output.is_none() && !relative) {
            error("not enough input or output options not provided", no_color);
        }

        // build in paths
        let in_paths = inputs
            .into_iter()
            .map(|i| PathBuf::from(i.expect("bad input")))
            .collect::<Vec<_>>();

        // check in paths
        for path in &in_paths {
            if !path.is_file() {
                error(format!("'{}' is not a file", path.display()), no_color);
            }
        }

        // build out path
        let out_dir_path = if relative {
            in_paths[0]
                .parent()
                .unwrap_or_else(|| Path::new("./"))
                .to_owned()
        } else {
            PathBuf::from(output.expect("bad output"))
        };
        let out_path = {
            let name = in_paths[0]
                .file_stem()
                .expect("bad stem")
                .to_str()
                .expect("bad str");
            out_dir_path.join(Path::new(&format!("{}.cubemap", name)))
        };
        fs::create_dir_all(&out_dir_path).expect("bad dir");

        // import image
        eprint!(
            "Converting {} and others ... ",
            in_paths[0]
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        io::stderr().lock().flush().expect("bad flush");

        let image_data = in_paths
            .iter()
            .map(|p| fs::read(p).expect("bad read"))
            .collect::<Vec<_>>();
        let binary = match import_cubemap(&image_data) {
            Ok(bin) => bin,
            Err(err) => error(format!("{}", err), no_color),
        };
        let mut out_file = File::create(&out_path).expect("bad file");
        out_file.write_all(&binary).expect("bad write");

        eprintln!("done");
    }
}

fn cubemap_help(no_color: bool) {
    eprintln!(
        r#"imports cubemap files (.png)

{}
    $ dai cubemap [OPTIONS]

{}
    -i, --in [files]         {}
    -o, --out [directory]   {}
    -r, --relative          {}

{}
    $ dai cubemap --relative --in top.png bottom.png front.png back.png left.png right.png
"#,
        title("USAGE", no_color),
        title("OPTIONS", no_color),
        desc(
            "input 6 image files in order top, bottom, front, back, left, right (.png)",
            no_color
        ),
        desc("output file directory", no_color),
        desc("use input file directory for output", no_color),
        title("EXAMPLE", no_color)
    );
}
