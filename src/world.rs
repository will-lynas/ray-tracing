use std::ops::Range;

use crate::{
    aabb::Aabb,
    color::Color,
    hittable::{
        HitRecord,
        Hittable,
    },
    material::Material,
    object::Object,
    timed_ray::TimedRay,
};

#[derive(Default, Clone)]
pub struct World {
    pub objects: Vec<Object>,
    bounding_box: Option<Aabb>,
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
        let hittable = hittable.into();
        let material = material.into();

        self.bounding_box = if let Some(bounding_box) = &self.bounding_box {
            Some(bounding_box.merge(&hittable.bounding_box()))
        } else {
            Some(hittable.bounding_box())
        };

        self.objects.push(Object { hittable, material });
    }
}
