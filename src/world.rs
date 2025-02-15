use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use std::ops::Range;

pub struct Object {
    pub hittable: Hittable,
    pub material: Material,
}

#[derive(Default)]
pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<(HitRecord, Material)> {
        let mut output = None;
        let mut check_interval = interval.clone();

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hittable.hit(r, &check_interval) {
                check_interval = check_interval.start..temp_record.t;
                output = Some((temp_record, object.material));
            }
        }

        output
    }

    pub fn bounce(&self, r: &Ray, interval: &Range<f64>) -> Option<(Ray, Color)> {
        let (hit_record, material) = self.hit(r, interval)?;
        material.scatter(&hit_record)
    }
}
