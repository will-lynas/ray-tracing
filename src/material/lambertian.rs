use glam::Vec3A as Vec3;

use super::Material;
use crate::{
    color::Color,
    extension_traits::Vec3Ext,
    hittable::HitRecord,
    texture::Texture,
    timed_ray::TimedRay,
};

#[derive(Debug)]
pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: impl Texture + 'static) -> Self {
        Self {
            texture: Box::new(texture),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_record: &HitRecord) -> Option<(TimedRay, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = TimedRay::new(hit_record.point, scatter_direction, hit_record.in_ray.time);
        let attenuation = self.texture.value(hit_record.uv, hit_record.point);
        Some((scattered, attenuation))
    }
}
