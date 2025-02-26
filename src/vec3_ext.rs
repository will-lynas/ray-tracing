use glam::Vec3;

use crate::rng::ThreadRng;

pub trait Vec3Ext {
    fn random_in_unit_disk() -> Self;
    fn random_unit_vector() -> Self;
    fn random() -> Self;
    fn near_zero(&self) -> bool;
    fn random_in_hemisphere(&self) -> Self;
}

impl Vec3Ext for Vec3 {
    fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                ThreadRng::random_range(-1.0..1.0),
                ThreadRng::random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    fn random_unit_vector() -> Self {
        loop {
            let v = Self::random();
            if (1e-160..=1.0).contains(&v.length_squared()) {
                return v.normalize();
            }
        }
    }

    fn random() -> Self {
        Self {
            x: ThreadRng::random_range(-1.0..1.0),
            y: ThreadRng::random_range(-1.0..1.0),
            z: ThreadRng::random_range(-1.0..1.0),
        }
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    fn random_in_hemisphere(&self) -> Self {
        let v = Self::random_unit_vector();
        if self.dot(v) > 0.0 {
            v
        } else {
            -v
        }
    }
}
