use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::ops::Range;

#[derive(Default)]
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut check_interval = interval.clone();

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hit(r, &check_interval) {
                check_interval = check_interval.start..temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }
}
