use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn front_face(normal: Vec3, r: &Ray) -> (bool, Vec3) {
        let front_face = normal.dot(&r.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
