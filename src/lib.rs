#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::many_single_char_names)]

mod aabb;
pub mod camera;
pub mod color;
pub mod extension_traits;
pub mod hittable;
pub mod material;
mod ray;
pub mod rng;
pub mod texture;
mod timed_ray;
