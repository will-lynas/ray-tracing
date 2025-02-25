use crate::{
    color::{
        Color,
        WHITE,
    },
    hittable::HitRecord,
    ray::Ray,
    rng::ThreadRng,
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
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_index) > ThreadRng::random()
        {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_index)
        };

        let scattered = Ray::new(hit_record.point, direction);
        Some((scattered, WHITE))
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}
