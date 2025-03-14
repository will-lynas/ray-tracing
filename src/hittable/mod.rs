use std::{
    fmt::Debug,
    ops::Range,
};

use glam::{
    Vec2,
    Vec3A as Vec3,
};

use crate::{
    aabb::Aabb,
    material::Material,
    timed_ray::TimedRay,
};
mod bvh_node;
mod list;
mod sphere;
pub use bvh_node::BvhNode;
pub use list::List as HittableList;
pub use sphere::Sphere;
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub uv: Vec2,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub in_ray: TimedRay,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn front_face(normal: Vec3, r: &TimedRay) -> (bool, Vec3) {
        let front_face = normal.dot(r.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        (front_face, normal)
    }
}

pub trait Hittable: Sync + Debug {
    fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
