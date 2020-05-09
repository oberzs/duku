mod error;
mod font;
mod sdf;
mod shader;

use clap::App;
use clap::Arg;
use std::path::Path;
use std::path::PathBuf;

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
                .short("d")
                .long("dir")
                .takes_value(true)
                .help("Input directory to import"),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .conflicts_with("dir")
                .takes_value(true)
                .help("Output file"),
        )
        .arg(
            Arg::with_name("out-dir")
                .long("out-dir")
                .conflicts_with("in")
                .takes_value(true)
                .help("Output directory"),
        )
        .get_matches();

    let input = opts.value_of("in").map(|p| Path::new(p));
    let dir = opts.value_of("dir").map(|p| Path::new(p));
    let output = opts.value_of("out").map(|p| Path::new(p));
    let output_dir = opts.value_of("out-dir").map(|p| Path::new(p));

    match (input, dir) {
        (Some(in_path), None) => {
            if !in_path.is_file() {
                panic!("input is not a file");
            }
            let def = default_out(in_path);
            let out_path = output.unwrap_or(&def);
            import_file(in_path, out_path).expect("cannot import file");
        }
        (None, Some(in_dir)) => {
            for entry in in_dir.read_dir().expect("dir is not a directory") {
                if let Ok(entry) = entry {
                    let in_path = entry.path();
                    if in_path.is_file() {
                        let dir = output_dir.unwrap_or_else(|| Path::new("."));
                        let def = default_out(&in_path);
                        let out_path = dir.join(def);
                        import_file(&in_path, &out_path).expect("cannot import file");
                    }
                }
            }
        }
        (_, _) => {}
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

fn default_out(in_path: &Path) -> PathBuf {
    let ext = in_path
        .extension()
        .map(|s| s.to_str().unwrap_or("out"))
        .unwrap_or("out");
    let name = in_path
        .file_stem()
        .map(|s| s.to_str().unwrap_or("output"))
        .unwrap_or("output");
    PathBuf::from(format!("{}.{}", name, ext))
}
