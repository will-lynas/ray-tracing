mod lambertian;
mod metal;
mod uniform;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use uniform::Uniform;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Uniform(Uniform),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(hit_record),
            Material::Uniform(uniform) => uniform.scatter(hit_record),
            Material::Metal(metal) => metal.scatter(hit_record),
        }
    }
}
