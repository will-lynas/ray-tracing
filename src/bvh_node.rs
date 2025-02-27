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

enum Children {
    One(Box<dyn Hittable>),
    Two(Box<dyn Hittable>, Box<dyn Hittable>),
}

pub struct BvhNode {
    children: Children,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Self {
        Self::new(list.objects)
    }

    pub fn new(_objects: Vec<Box<dyn Hittable>>) -> Self {
        todo!()
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord> {
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

    fn bounding_box(&self) -> Aabb {
        self.bounding_box.clone()
    }
}
