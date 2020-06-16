// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

//! Tegne Import is a utility for converting common file formats
//! to formats used in Tegne (e.g. shaders and fonts)

#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    box_pointers
)]

// should be imported first
mod macros;

mod error;
mod font;
mod sdf;
mod shader;

use clap::App;
use clap::Arg;
use crossbeam_channel::unbounded;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use error::Result;
use font::import_font;
use shader::import_shader;

fn main() {
    let opts = App::new("Tegne importer")
        .version("0.1.0")
        .author("Oliver Berzs <oliver.berzs@gmail.com>")
        .arg(
            Arg::with_name("in")
                .takes_value(true)
                .required_unless("dir")
                .help("Input file to import"),
        )
        .arg(
            Arg::with_name("dir")
                .long("dir")
                .short("d")
                .takes_value(true)
                .help("Input directory to import"),
        )
        .arg(
            Arg::with_name("out-dir")
                .long("out-dir")
                .short("o")
                .takes_value(true)
                .help("Output directory"),
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .short("w")
                .help("Watch for file changes"),
        )
        .get_matches();

    let input = opts.value_of("in").map(|p| Path::new(p));
    let dir = opts.value_of("dir").map(|p| Path::new(p));
    let out_dir = opts
        .value_of("out-dir")
        .map(|p| Path::new(p))
        .unwrap_or_else(|| Path::new("."));
    let watch = opts.is_present("watch");

    // import input file
    if let Some(in_path) = input {
        if !in_path.is_file() {
            error!("'{}' is not a file", in_path.display());
        }
        let out_path = create_out_path(in_path, out_dir);
        if let Err(err) = import_file(in_path, &out_path) {
            error!("{}", err);
        }
    }

    // import files from input directory
    if let Some(in_dir) = dir {
        let entries = match in_dir.read_dir() {
            Ok(value) => value,
            _ => error!("'{}' is not a directory", in_dir.display()),
        };
        for entry in entries {
            if let Ok(entry) = entry {
                let in_path = entry.path();
                if in_path.is_file() {
                    let out_path = create_out_path(&in_path, out_dir);
                    if let Err(err) = import_file(&in_path, &out_path) {
                        error!("{}", err);
                    }
                }
            }
        }
    }

    // watch for changes
    if watch {
        let path = input.or(dir).unwrap();
        let (sender, receiver) = unbounded();
        let start_time = Instant::now();

        let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
            let time = start_time.elapsed().as_secs();
            match res {
                Err(err) => error!("{}", err),
                Ok(r) => sender.send((r, time)).unwrap(),
            }
        })
        .unwrap();
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

        let mut same_events = HashSet::new();
        loop {
            let (event, time) = receiver.recv().unwrap();
            let in_path = event.paths[0].clone();

            // limit events
            if !same_events.contains(&(in_path.clone(), time)) {
                same_events.insert((in_path.clone(), time));

                // wait to commit
                thread::sleep(Duration::from_millis(500));

                let out_path = create_out_path(&in_path, out_dir);
                if let Err(err) = import_file(&in_path, &out_path) {
                    warn!("{}", err);
                }
            }
        }
    }
}

fn import_file(in_path: &Path, out_path: &Path) -> Result<()> {
    let path_str = in_path.to_str().unwrap_or_default();
    if path_str.ends_with(".glsl") {
        import_shader(in_path, out_path)?;
    }
    if path_str.ends_with(".ttf") {
        import_font(in_path, out_path)?;
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
