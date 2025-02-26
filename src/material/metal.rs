use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Option<Self> {
        (0.0..=1.0).contains(&fuzz).then_some(Self { albedo, fuzz })
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = hit_record.in_ray.direction.reflect(&hit_record.normal);
        reflected = reflected.unit_vector() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(hit_record.point, reflected);
        (scattered.direction.dot(&hit_record.normal) > 0.0).then_some((scattered, self.albedo))
    }
}
