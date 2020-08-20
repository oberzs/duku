// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// font subcommand

mod bitmap;
mod diamond_iterator;
mod import;
mod sdf;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use super::desc;
use super::error;
use super::title;
use import::import_font;
use import::FontOptions;

pub fn font(mut args: Vec<String>, help: bool, no_color: bool) {
    if help {
        font_help(no_color);
    } else {
        // gather arguments
        let relative = args
            .iter()
            .position(|a| *a == "--relative" || *a == "-r")
            .map(|pos| args.remove(pos))
            .is_some();

        let mut input = None;
        let mut output = None;
        let mut bitmaps = None;
        let mut sdf = None;
        let mut iter_args = args.into_iter();
        while let Some(arg) = iter_args.next() {
            match arg.as_str() {
                "--in" | "-i" => input = iter_args.next(),
                "--out" | "-o" => output = iter_args.next(),
                "--bitmaps" | "-b" => bitmaps = iter_args.next(),
                "--sdf" | "-s" => sdf = iter_args.next(),
                _ => error(format!("unknown argument '{}'", arg), no_color),
            }
        }

        // check arguments
        if input.is_none() || (output.is_none() && !relative) {
            error("input and output options not provided", no_color);
        }

        let bitmap_sizes = match bitmaps
            .unwrap_or_else(|| "18,24,32".to_string())
            .split(',')
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
        {
            Err(_) => error("invalid bitmap sizes", no_color),
            Ok(s) => s,
        };

        let sdf_params = match sdf
            .unwrap_or_else(|| "1024,64,8".to_string())
            .split(',')
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
        {
            Err(_) => error("invalid sdf parameters", no_color),
            Ok(s) if s.len() < 3 => error("not enough sdf parameters", no_color),
            Ok(s) => s,
        };

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
            out_dir_path.join(Path::new(&format!("{}.font", name)))
        };
        fs::create_dir_all(&out_dir_path).expect("bad dir");

        // import font
        eprint!(
            "Converting {} ... ",
            in_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        io::stderr().lock().flush().expect("bad flush");

        let font_data = fs::read(&in_path).expect("bad read");
        let binary = match import_font(
            &font_data,
            FontOptions {
                bitmap_sizes: &bitmap_sizes,
                sdf_sample: sdf_params[0],
                sdf_size: sdf_params[1],
                sdf_margin: sdf_params[2] as u16,
            },
        ) {
            Ok(bin) => bin,
            Err(err) => error(format!("{}", err), no_color),
        };
        let mut out_file = File::create(&out_path).expect("bad file");
        out_file.write_all(&binary).expect("bad write");

        eprintln!("done");
    }
}

fn font_help(no_color: bool) {
    eprintln!(
        r#"imports font file (.ttf)

{}
    $ dai font [OPTIONS]

{}
    -i, --in [file]         {}
    -o, --out [directory]   {}
    -r, --relative          {}
    -b, --bitmaps           {}
    -s, --sdf               {}

{}
    $ dai font --relative --in font.ttf
"#,
        title("USAGE", no_color),
        title("OPTIONS", no_color),
        desc("input font file (.ttf)", no_color),
        desc("output file directory", no_color),
        desc("use input file directory for output", no_color),
        desc("bitmap sizes separated by commas (ex. 18,24,32)", no_color),
        desc(
            "sdf sample size, font size and margin separated by commas (ex. 1024,64,8)",
            no_color
        ),
        title("EXAMPLE", no_color)
    );
}
