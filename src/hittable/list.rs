use std::ops::Range;

use crate::{
    aabb::Aabb,
    color::Color,
    hittable::{
        HitRecord,
        Hittable,
    },
    timed_ray::TimedRay,
};

#[derive(Default, Clone)]
pub struct List {
    pub objects: Vec<Hittable>,
    bounding_box: Option<Aabb>,
}

impl List {
    fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<HitRecord> {
        let mut output = None;
        let mut check_interval = interval.clone();

        for object in &self.objects {
            if let Some(temp_record) = object.hit(r, &check_interval) {
                check_interval = check_interval.start..temp_record.t;
                output = Some(temp_record);
            }
        }

        output
    }

    pub fn bounce(&self, r: &TimedRay, interval: &Range<f32>) -> Option<(TimedRay, Color)> {
        let hit_record = self.hit(r, interval)?;
        hit_record.material.scatter(&hit_record)
    }

    pub fn add(&mut self, hittable: impl Into<Hittable>) {
        let hittable = hittable.into();

        self.bounding_box = if let Some(bounding_box) = &self.bounding_box {
            Some(bounding_box.merge(&hittable.bounding_box()))
        } else {
            Some(hittable.bounding_box())
        };

        self.objects.push(hittable);
    }
}
