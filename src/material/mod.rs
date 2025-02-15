mod lambertian;
pub use lambertian::Lambertian;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
}

impl Material {
    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(hit_record),
        }
    }
}
