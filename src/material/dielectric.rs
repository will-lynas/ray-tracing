use crate::{
    camera::Stores,
    color::{
        Color,
        WHITE,
    },
    extension_traits::Vec3Ext,
    hittable::HitRecord,
    material::Material,
    timed_ray::TimedRay,
};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &HitRecord, _stores: &Stores) -> Option<(TimedRay, Color)> {
        let refraction_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = hit_record.in_ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_index) > fastrand::f32() {
                unit_direction.reflect(hit_record.normal)
            } else {
                unit_direction.refract_custom(hit_record.normal, refraction_index)
            };

        let scattered = TimedRay::new(hit_record.point, direction, hit_record.in_ray.time);
        Some((scattered, WHITE))
    }
}
