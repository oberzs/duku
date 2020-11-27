// Oliver Berzs
// https://github.com/oberzs/duku

mod gradient;
mod hsb;
mod mix;
mod rgb;
mod rgbf;

use mix::mixf;

pub use gradient::Gradient;
pub use hsb::Hsb;
pub use mix::Mix;
pub use rgb::Rgb;
pub use rgbf::Rgbf;
