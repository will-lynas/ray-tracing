#![allow(dead_code)]

use std::ops::Range;

use crate::{
    aabb::Aabb,
    hittable::{
        HitRecord,
        Hittable,
        HittableList,
    },
    timed_ray::TimedRay,
};

pub struct BvhNode {
    left: Hittable,
    right: Hittable,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new_from_list(_world: &HittableList) -> Self {
        todo!()
    }

    pub fn new_from_objects(_objects: &[Hittable]) -> Self {
        todo!()
    }

    pub fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, interval) {
            return None;
        }

        if let Some(hit_record) = self.left.hit(r, interval) {
            let interval = interval.start..hit_record.t;
            let hit_right = self.right.hit(r, &interval);
            if let Some(hit_record) = hit_right {
                Some(hit_record)
            } else {
                Some(hit_record)
            }
        } else {
            None
        }
    }
}
