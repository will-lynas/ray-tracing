use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
};

#[derive(Clone, Copy)]
pub struct Uniform {
    albedo: Color,
}

impl Uniform {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal.random_in_hemisphere();
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}
