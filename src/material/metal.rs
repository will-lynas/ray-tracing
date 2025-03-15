use glam::Vec3A as Vec3;

use super::Material;
use crate::{
    camera::Stores,
    color::Color,
    extension_traits::Vec3Ext,
    hittable::HitRecord,
    timed_ray::TimedRay,
};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        assert!((0.0..=1.0).contains(&fuzz));
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord, _stores: &Stores) -> Option<(TimedRay, Color)> {
        let mut reflected = hit_record.in_ray.direction.reflect(hit_record.normal);
        reflected = reflected.normalize() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = TimedRay::new(hit_record.point, reflected, hit_record.in_ray.time);
        (scattered.direction.dot(hit_record.normal) > 0.0).then_some((scattered, self.albedo))
    }
}
