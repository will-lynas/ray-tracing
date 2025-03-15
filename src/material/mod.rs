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
    camera::Stores,
    color::Color,
    hittable::HitRecord,
    timed_ray::TimedRay,
};

pub trait Material: Sync + Debug {
    // TODO: Passing in stores is pretty bad, but it works for now
    fn scatter(&self, hit_record: &HitRecord, stores: &Stores) -> Option<(TimedRay, Color)>;
}
