use std::{
    mem::swap,
    ops::Range,
};

use glam::Vec3A as Vec3;

use crate::{
    timed_ray::TimedRay,
    vec3_ext::Vec3Ext,
};

pub struct Aabb {
    x: Range<f32>,
    y: Range<f32>,
    z: Range<f32>,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            x: a.x.min(b.x)..a.x.max(b.x),
            y: a.y.min(b.y)..a.y.max(b.y),
            z: a.z.min(b.z)..a.z.max(b.z),
        }
    }

    pub fn axis(&self, axis: u8) -> Range<f32> {
        match axis {
            0 => self.x.clone(),
            1 => self.y.clone(),
            2 => self.z.clone(),
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, r: TimedRay, mut ray_t: Range<f32>) -> bool {
        for i in 0..3 {
            let ax = self.axis(i);
            let d_inv = 1.0 / r.direction.axis(i);

            let mut t0 = (ax.start - r.origin.axis(i)) * d_inv;
            let mut t1 = (ax.end - r.origin.axis(i)) * d_inv;
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.start {
                ray_t.start = t0;
            }
            if t1 < ray_t.end {
                ray_t.end = t1;
            }
            if ray_t.end <= ray_t.start {
                return false;
            }
        }
        true
    }
}
