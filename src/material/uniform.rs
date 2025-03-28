use super::Material;
use crate::{
    camera::Stores,
    color::Color,
    extension_traits::Vec3Ext,
    hittable::HitRecord,
    timed_ray::TimedRay,
};

#[derive(Debug)]
pub struct Uniform {
    albedo: Color,
}

impl Uniform {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Uniform {
    fn scatter(&self, hit_record: &HitRecord, _stores: &Stores) -> Option<(TimedRay, Color)> {
        let scatter_direction = hit_record.normal.random_in_hemisphere();
        let scattered = TimedRay::new(hit_record.point, scatter_direction, hit_record.in_ray.time);
        Some((scattered, self.albedo))
    }
}
