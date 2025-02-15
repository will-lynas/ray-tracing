use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}
