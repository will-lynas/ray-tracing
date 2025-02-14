use crate::color::Color;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn color(&self) -> Color {
        Color::new(0.9, 0.1, 0.9).unwrap()
    }
}
