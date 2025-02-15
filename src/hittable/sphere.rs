use std::ops::Range;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Option<Self> {
        (radius > 0.0).then_some(Self { center, radius })
    }

    pub fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let root = (h - sqrt_d) / a;
        let t = if root > interval.start && root < interval.end {
            root
        } else {
            let root2 = (h + sqrt_d) / a;
            if root2 > interval.start && root2 < interval.end {
                root2
            } else {
                return None;
            }
        };

        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let (front_face, normal) = HitRecord::front_face(outward_normal, r);
        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            in_ray: *r,
        })
    }
}
