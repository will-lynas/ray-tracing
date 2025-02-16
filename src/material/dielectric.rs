use crate::{
    color::{
        Color,
        WHITE,
    },
    hittable::HitRecord,
    ray::Ray,
};

#[derive(Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = hit_record.in_ray.direction.unit_vector();
        let refracted = unit_direction.refract(&hit_record.normal, refraction_index);
        let scattered = Ray::new(hit_record.point, refracted);
        Some((scattered, WHITE))
    }
}
