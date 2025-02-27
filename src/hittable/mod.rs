use std::ops::Range;

use glam::Vec3A as Vec3;

use crate::{
    aabb::Aabb,
    timed_ray::TimedRay,
};
mod sphere;
pub use sphere::Sphere;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub in_ray: TimedRay,
}

impl HitRecord {
    pub fn front_face(normal: Vec3, r: &TimedRay) -> (bool, Vec3) {
        let front_face = normal.dot(r.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        (front_face, normal)
    }
}

#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
}

impl Hittable {
    pub fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(r, interval),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.bounding_box.clone(),
        }
    }
}

impl From<Sphere> for Hittable {
    fn from(sphere: Sphere) -> Self {
        Self::Sphere(sphere)
    }
}
