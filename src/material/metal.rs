use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = hit_record.in_ray.direction.reflect(&hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        Some((scattered, self.albedo))
    }
}
