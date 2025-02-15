use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::Vec3,
};

mod sphere;
pub use sphere::Sphere;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub in_ray: Ray,
}

impl HitRecord {
    pub fn front_face(normal: Vec3, r: &Ray) -> (bool, Vec3) {
        let front_face = normal.dot(&r.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        (front_face, normal)
    }
}

pub enum Hittable {
    Sphere(Sphere),
}

impl Hittable {
    pub fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(r, interval),
        }
    }
}

impl From<Sphere> for Hittable {
    fn from(sphere: Sphere) -> Self {
        Self::Sphere(sphere)
    }
}
