use crate::color::Color;
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

    pub fn color(&self, r: &Ray) -> Color {
        let interval = 0.0..f64::MAX;
        if let Some(hit_record) = self.hit(r, &interval) {
            Color::from_unit_vector(hit_record.normal).unwrap()
        } else {
            self.background(r)
        }
    }

    pub fn background(&self, r: &Ray) -> Color {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::white().lerp(&Color::blue(), t).unwrap()
    }
}
