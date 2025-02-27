use std::ops::Range;

use crate::{
    aabb::Aabb,
    hittable::{
        HitRecord,
        Hittable,
    },
    timed_ray::TimedRay,
};

#[derive(Default, Debug)]
pub struct List {
    pub objects: Vec<Box<dyn Hittable>>,
    bounding_box: Aabb,
}

impl List {
    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.bounding_box = self.bounding_box.merge(&hittable.bounding_box());
        self.objects.push(Box::new(hittable));
    }
}

impl Hittable for List {
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

    fn bounding_box(&self) -> Aabb {
        self.bounding_box.clone()
    }
}
