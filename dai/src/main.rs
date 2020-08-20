// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

//! Draw-it Asset Importer is a utility for converting common file formats
//! to formats used in Draw-it (e.g. shaders and fonts)

#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    // clippy::missing_const_for_fn,
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::if_not_else,
    clippy::imprecise_flops,
    clippy::inefficient_to_string
)]

mod cubemap;
mod error;
mod font;
mod image;
mod shader;
mod watch;

use std::env;
use std::process;

use cubemap::cubemap;
use font::font;
use image::image;
use shader::shader;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    // check if no arguments
    if args.is_empty() {
        show_help(false);
        return;
    }

    // check if subcommand exists
    let subcommand = if args[0].starts_with('-') {
        None
    } else {
        Some(args.remove(0))
    };

    // check for unpositional arguments
    let no_color = args
        .iter()
        .position(|a| *a == "--no-color")
        .map(|pos| args.remove(pos))
        .is_some();
    let help = args
        .iter()
        .position(|a| *a == "--help" || *a == "-h")
        .map(|pos| args.remove(pos))
        .is_some();

    // perform subcommand
    if let Some(name) = subcommand {
        match name.as_str() {
            "version" => println!("{}", VERSION),
            "shader" => shader(args, help, no_color),
            "font" => font(args, help, no_color),
            "image" => image(args, help, no_color),
            "cubemap" => cubemap(args, help, no_color),
            _ => show_help(no_color),
        }
    } else {
        // display help
        show_help(no_color);
    }
}

fn show_help(no_color: bool) {
    eprintln!(
        r#"Draw-it Asset Importer
    
{}
    {}

{}
    $ dai [COMMAND] [OPTIONS]

{}
    version     {}
    shader      {}
    font        {}
    image       {}
    cubemap     {}

{}
    -h, --help  {}
    --no-color  {}
    "#,
        title("VERSION", no_color),
        VERSION,
        title("USAGE", no_color),
        title("COMMANDS", no_color),
        desc("outputs dai version", no_color),
        desc("import shader file (.glsl)", no_color),
        desc("import font file (.ttf)", no_color),
        desc("import image file (.png)", no_color),
        desc("import 6 image files to create a cubemap (.png)", no_color),
        title("OPTIONS", no_color),
        desc("shows help about specific command", no_color),
        desc("disables color for output", no_color)
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

fn warn(s: impl AsRef<str>, no_color: bool) {
    if no_color {
        eprintln!("warning: {}", s.as_ref());
    } else {
        eprintln!("\x1b[93mwarning\x1b[0m: {}", s.as_ref());
    }
}

fn title(s: &str, no_color: bool) -> String {
    format!("{}{}{}", bold(no_color), s, clear(no_color))
}

fn desc(s: &str, no_color: bool) -> String {
    format!("{}{}{}", dim(no_color), s, clear(no_color))
}

fn clear(no_color: bool) -> &'static str {
    if no_color {
        ""
    } else {
        "\x1B[0m"
    }
}

fn bold(no_color: bool) -> &'static str {
    if no_color {
        ""
    } else {
        "\x1B[97m"
    }
}

fn dim(no_color: bool) -> &'static str {
    if no_color {
        ""
    } else {
        "\x1B[90m"
    }
}
