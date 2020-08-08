// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

//! Draw-it Import is a utility for converting common file formats
//! to formats used in Draw-it (e.g. shaders and fonts)

#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]

// should be imported first
mod macros;

mod bitmap;
mod diamond_iterator;
mod error;
mod font;
mod sdf;
mod shader;
mod watch;

use clap::App;
use clap::Arg;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

use error::Result;
use font::import_font;
use font::FontOptions;
use shader::import_shader;
use watch::watch_file;

fn main() {
    let opts = App::new("Draw-it importer")
        .version("0.1.0")
        .author("Oliver Berzs <oliver.berzs@gmail.com>")
        .arg(
            Arg::with_name("in")
                .takes_value(true)
                .required_unless("directory")
                .help("Input file to import"),
        )
        .arg(
            Arg::with_name("out")
                .long("out")
                .short("o")
                .takes_value(true)
                .help("Output directory"),
        )
        .arg(
            Arg::with_name("directory")
                .long("directory")
                .short("d")
                .takes_value(true)
                .help("Input directory to import"),
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .short("w")
                .conflicts_with("directory")
                .help("Watch for file changes"),
        )
        .arg(
            Arg::with_name("relative")
                .long("relative")
                .short("r")
                .conflicts_with("out-dir")
                .help("Uses input directory as output directory"),
        )
        .arg(
            Arg::with_name("sdf")
                .long("sdf")
                .short("s")
                .takes_value(true)
                .default_value("4096,64,8")
                .help("If importing font, sets SDF format options separated by commas 'sample-size,font-size,font-margin'"),

        ).arg(
            Arg::with_name("bitmap")
                .long("bitmap")
                .short("b")
                .takes_value(true)
                .multiple(true)
                .default_value("18,24,32")
                .help("If importing font, sets bitmap format sizes")
        ).get_matches();

    let input = opts.value_of("in").map(|p| Path::new(p));
    let out = opts.value_of("out").map(|p| Path::new(p));
    let directory = opts.value_of("directory").map(|p| Path::new(p));
    let watch = opts.is_present("watch");
    let relative = opts.is_present("relative");
    let sdf = opts.value_of("sdf").expect("bad args");
    let bitmap = opts.values_of("bitmap").expect("bad args");

    let sdf_options = sdf
        .split(',')
        .map(|s| match s.parse::<u32>() {
            Ok(n) => n,
            _ => error!("'{}' is not a number", s),
        })
        .collect::<Vec<_>>();
    if sdf_options.len() != 3 {
        error!("sdf option should have 3 values");
    }
    let mut bitmap_options = vec![];
    bitmap.for_each(|s| {
        s.split(',').for_each(|s| match s.parse::<u32>() {
            Ok(n) => bitmap_options.push(n),
            _ => error!("'{}' is not a number", s),
        })
    });

    let out_dir = if relative {
        if let Some(dir) = directory {
            dir
        } else if let Some(i) = input {
            i.parent().unwrap_or_else(|| Path::new("."))
        } else {
            error!("something went wrong");
        }
    } else {
        out.unwrap_or_else(|| Path::new("."))
    };

    // import input file
    if let Some(in_path) = input {
        if !in_path.is_file() {
            error!("'{}' is not a file", in_path.display());
        }
        let out_path = create_out_path(in_path, out_dir);
        if let Err(err) = import_file(in_path, &out_path, &sdf_options, &bitmap_options) {
            error!("{}", err);
        }
    }

    // import files from input directory
    if let Some(in_dir) = directory {
        let entries = match in_dir.read_dir() {
            Ok(value) => value,
            _ => error!("'{}' is not a directory", in_dir.display()),
        };
        for entry in entries {
            if let Ok(entry) = entry {
                let in_path = entry.path();
                if in_path.is_file() {
                    let out_path = create_out_path(&in_path, out_dir);
                    if let Err(err) =
                        import_file(&in_path, &out_path, &sdf_options, &bitmap_options)
                    {
                        error!("{}", err);
                    }
                }
            }
        }
    }

    // watch for changes
    if watch {
        let path = input.expect("bad args");
        let (sender, receiver) = mpsc::channel();

        watch_file(path, sender);

        while let Ok(in_path) = receiver.recv() {
            let out_path = create_out_path(&in_path, out_dir);
            if let Err(err) = import_file(&in_path, &out_path, &sdf_options, &bitmap_options) {
                warn!("{}", err);
            }
        }
    }
}

fn import_file(
    in_path: &Path,
    out_path: &Path,
    sdf_options: &[u32],
    bitmap_options: &[u32],
) -> Result<()> {
    let path_str = in_path.to_str().unwrap_or_default();
    if path_str.ends_with(".glsl") {
        import_shader(in_path, out_path)?;
    }
    if path_str.ends_with(".ttf") {
        import_font(
            in_path,
            out_path,
            FontOptions {
                sdf_sample: sdf_options[0],
                sdf_size: sdf_options[1],
                sdf_margin: sdf_options[2] as u16,
                bitmap_sizes: bitmap_options,
            },
        )?;
    }
    Ok(())
}

fn create_out_path(in_path: &Path, out_dir: &Path) -> PathBuf {
    let ext = in_path
        .extension()
        .map(|s| s.to_str().unwrap_or("out"))
        .unwrap_or("out");
    let name = in_path
        .file_stem()
        .map(|s| s.to_str().unwrap_or("output"))
        .unwrap_or("output");
    let default_out = format!("{}.{}", name, ext);
    let default_path = Path::new(&default_out);

    out_dir.join(default_path)
}
