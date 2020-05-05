use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Props {
    x_offset: i32,
    y_offset: i32,
    chars: Vec<Bounds>,
}

#[derive(Deserialize)]
struct Bounds {
    bound_x: f32,
    bound_y: f32,
    bound_w: f32,
    bound_h: f32,
    advance: f32,
    range: i32,
    #[serde(rename = "char")]
    symbol: char,
}

#[derive(Deserialize)]
pub(crate) struct Atlas {
    frames: Vec<Frame>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Frame {
    filename: String,
    frame: PosSize,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: PosSize,
    #[serde(rename = "sourceSize")]
    source_size: Size,
}

#[derive(Deserialize)]
struct Meta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: Size,
    scale: String,
    smartupdate: String,
}

#[derive(Deserialize)]
struct PosSize {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct Size {
    w: u32,
    h: u32,
}
