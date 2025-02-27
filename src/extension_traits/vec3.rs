use std::ops::Range;

use glam::Vec3A as Vec3;

use crate::rng::ThreadRng;

pub trait Vec3Ext {
    fn random_in_unit_disk() -> Self;
    fn random_unit_vector() -> Self;
    fn random() -> Self;
    fn random_range(range: &Range<f32>) -> Self;
    fn near_zero(&self) -> bool;
    fn random_in_hemisphere(&self) -> Self;
    fn refract_custom(&self, normal: Self, etai_over_etat: f32) -> Self;
    fn axis(&self, axis: usize) -> f32;
}

impl Vec3Ext for Vec3 {
    fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                ThreadRng::random_range(&(-1.0..1.0)),
                ThreadRng::random_range(&(-1.0..1.0)),
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
        Self::random_range(&(0.0..1.0))
    }

    fn random_range(range: &Range<f32>) -> Self {
        Self::new(
            ThreadRng::random_range(range),
            ThreadRng::random_range(range),
            ThreadRng::random_range(range),
        )
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

    fn refract_custom(&self, normal: Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    fn axis(&self, axis: usize) -> f32 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }
}
