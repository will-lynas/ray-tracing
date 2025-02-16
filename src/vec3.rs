use std::ops::{
    Add,
    Div,
    Mul,
    Neg,
    Sub,
};

use rand::Rng;

pub const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(-1.0..=1.0),
            y: rng.random_range(-1.0..=1.0),
            z: rng.random_range(-1.0..=1.0),
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let v = Self::random();
            if (1e-160..=1.0).contains(&v.length_squared()) {
                return v.unit_vector();
            }
        }
    }

    pub fn random_in_hemisphere(&self) -> Self {
        let v = Self::random_unit_vector();
        if self.dot(&v) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * self.dot(normal) * 2.0
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
