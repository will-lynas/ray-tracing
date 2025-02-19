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
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;
        let refracted = if cannot_refract {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_index)
        };

        let scattered = Ray::new(hit_record.point, refracted);
        Some((scattered, WHITE))
    }
}
