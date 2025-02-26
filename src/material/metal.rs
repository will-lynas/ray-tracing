use glam::Vec3A as Vec3;

use crate::{
    color::Color,
    hittable::HitRecord,
    timed_ray::TimedRay,
    vec3_ext::Vec3Ext,
};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        assert!((0.0..=1.0).contains(&fuzz));
        Self { albedo, fuzz }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(TimedRay, Color)> {
        let mut reflected = hit_record.in_ray.direction.reflect(hit_record.normal);
        reflected = reflected.normalize() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = TimedRay::new(hit_record.point, reflected, hit_record.in_ray.time);
        (scattered.direction.dot(hit_record.normal) > 0.0).then_some((scattered, self.albedo))
    }
}
