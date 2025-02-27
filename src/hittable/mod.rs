use std::ops::Range;

use glam::Vec3A as Vec3;

use crate::{
    aabb::Aabb,
    material::Material,
    timed_ray::TimedRay,
};
mod list;
mod sphere;
pub use list::List as HittableList;
pub use sphere::Sphere;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub in_ray: TimedRay,
    pub material: Material,
}

impl HitRecord {
    pub fn front_face(normal: Vec3, r: &TimedRay) -> (bool, Vec3) {
        let front_face = normal.dot(r.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        (front_face, normal)
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
