use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }

    pub fn color(&self, r: &Ray) -> Color {
        if let Some(hit_record) = self.hit(r, 0.0, f64::MAX) {
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
