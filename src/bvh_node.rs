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

enum Children<'a> {
    One(&'a dyn Hittable),
    Two(&'a dyn Hittable, &'a dyn Hittable),
}

pub struct BvhNode<'a> {
    children: Children<'a>,
    bounding_box: Aabb,
}

impl<'a> BvhNode<'a> {
    pub fn new_from_list(_world: &HittableList) -> Self {
        todo!()
    }

    pub fn new_from_objects(_objects: &[&'a dyn Hittable]) -> Self {
        todo!()
    }

    pub fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, interval) {
            return None;
        }

        match &self.children {
            Children::One(hittable) => hittable.hit(r, interval),
            Children::Two(left, right) => {
                if let Some(left_hit_record) = left.hit(r, interval) {
                    let interval = interval.start..left_hit_record.t;
                    if let Some(right_hit_record) = right.hit(r, &interval) {
                        Some(right_hit_record)
                    } else {
                        Some(left_hit_record)
                    }
                } else {
                    None
                }
            }
        }
    }
}
