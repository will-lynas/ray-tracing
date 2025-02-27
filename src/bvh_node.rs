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

#[derive(Debug)]
enum Children {
    One(Box<dyn Hittable>),
    Two(Box<dyn Hittable>, Box<dyn Hittable>),
}

#[derive(Debug)]
pub struct BvhNode {
    children: Children,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Self {
        Self::new(list.objects)
    }

    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let children = match objects.len() {
            0 => panic!("No objects to create BVH node from"),
            1 => Children::One(objects.remove(0)),
            2 => Children::Two(objects.remove(0), objects.remove(0)),
            _ => {
                let comparator = Aabb::random_axis_comparator();
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let left = Self::new(objects.drain(..mid).collect());
                let right = Self::new(objects);
                Children::Two(Box::new(left), Box::new(right))
            }
        };

        let bounding_box = match &children {
            Children::One(hittable) => hittable.bounding_box(),
            Children::Two(left, right) => left.bounding_box().merge(&right.bounding_box()),
        };

        Self {
            children,
            bounding_box,
        }
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
