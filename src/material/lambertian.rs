use glam::Vec3A as Vec3;

use crate::{
    color::Color,
    hittable::HitRecord,
    timed_ray::TimedRay,
    vec3_ext::Vec3Ext,
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(TimedRay, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = TimedRay::new(hit_record.point, scatter_direction, hit_record.in_ray.time);
        Some((scattered, self.albedo))
    }
}
