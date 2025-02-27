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
mod extension_traits;
pub mod hittable;
pub mod material;
mod object;
mod ray;
mod rng;
mod timed_ray;
pub mod world;
