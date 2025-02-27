mod dielectric;
mod lambertian;
mod metal;
mod uniform;

use std::fmt::Debug;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use uniform::Uniform;

use crate::{
    color::Color,
    hittable::HitRecord,
    timed_ray::TimedRay,
};

pub trait Material: Sync + Debug {
    fn scatter(&self, hit_record: &HitRecord) -> Option<(TimedRay, Color)>;
}
