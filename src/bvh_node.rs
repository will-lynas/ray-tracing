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
                let left_hit = left.hit(r, interval);
                let right_hit = match &left_hit {
                    Some(hit_record) => {
                        let new_interval = interval.start..hit_record.t;
                        right.hit(r, &new_interval)
                    }
                    None => right.hit(r, interval),
                };

                if right_hit.is_some() {
                    right_hit
                } else {
                    left_hit
                }
            }
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box.clone()
    }
}
