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

mod error;
mod font;
mod sdf;
mod shader;

use clap::App;
use clap::Arg;
use notify::DebouncedEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

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

    match (input, dir) {
        (Some(in_path), None) => {
            if !in_path.is_file() {
                panic!("input is not a file");
            }
            let out_path = create_out_path(in_path, out_dir);
            import_file(in_path, &out_path).expect("cannot import file");
        }
        (None, Some(in_dir)) => {
            for entry in in_dir.read_dir().expect("dir is not a directory") {
                if let Ok(entry) = entry {
                    let in_path = entry.path();
                    if in_path.is_file() {
                        let out_path = create_out_path(&in_path, out_dir);
                        import_file(&in_path, &out_path).expect("cannot import file");
                    }
                }
            }
        }
        (_, _) => {}
    }

    // watch for changes
    if watch {
        let path = input.or(dir).expect("no path given");
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Duration::from_secs(1)).expect("cannot watch system");
        watcher
            .watch(path, RecursiveMode::NonRecursive)
            .expect("cannot watch path");

        loop {
            let event = rx.recv().unwrap();
            if let DebouncedEvent::NoticeWrite(in_path) = event {
                let out_path = create_out_path(&in_path, out_dir);
                import_file(&in_path, &out_path).expect("cannot import file");
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
