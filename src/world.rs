use std::ops::Range;

use crate::{
    color::Color,
    hittable::{
        HitRecord,
        Hittable,
    },
    material::Material,
    timed_ray::TimedRay,
};

#[derive(Clone)]
pub struct Object {
    pub hittable: Hittable,
    pub material: Material,
}

#[derive(Default, Clone)]
pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    fn hit(&self, r: &TimedRay, interval: &Range<f32>) -> Option<(HitRecord, Material)> {
        let mut output = None;
        let mut check_interval = interval.clone();

        for object in &self.objects {
            if let Some(temp_record) = object.hittable.hit(r, &check_interval) {
                check_interval = check_interval.start..temp_record.t;
                output = Some((temp_record, object.material));
            }
        }

        output
    }

    pub fn bounce(&self, r: &TimedRay, interval: &Range<f32>) -> Option<(TimedRay, Color)> {
        let (hit_record, material) = self.hit(r, interval)?;
        material.scatter(&hit_record)
    }

    pub fn add(&mut self, hittable: impl Into<Hittable>, material: impl Into<Material>) {
        self.objects.push(Object {
            hittable: hittable.into(),
            material: material.into(),
        });
    }
}
