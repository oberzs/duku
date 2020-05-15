#![feature(proc_macro_hygiene)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    box_pointers
)]

mod camera;
mod matrix_4;
mod quaternion;
mod transform;
mod utils;
mod vector_2;
mod vector_3;
mod vector_4;

pub use camera::Camera;
pub use matrix_4::Matrix4;
pub use quaternion::Quaternion;
pub use transform::Transform;
pub use utils::clamp;
pub use vector_2::Vector2;
pub use vector_3::Vector3;
pub use vector_4::Vector4;
