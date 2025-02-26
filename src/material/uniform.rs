use crate::{
    color::Color,
    hittable::HitRecord,
    timed_ray::TimedRay,
    vec3_ext::Vec3Ext,
};

#[derive(Clone, Copy)]
pub struct Uniform {
    albedo: Color,
}

impl Uniform {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(TimedRay, Color)> {
        let scatter_direction = hit_record.normal.random_in_hemisphere();
        let scattered = TimedRay::new(hit_record.point, scatter_direction, hit_record.in_ray.time);
        Some((scattered, self.albedo))
    }
}
