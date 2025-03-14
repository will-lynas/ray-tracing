use glam::{
    Vec2,
    Vec3A as Vec3,
};

use crate::color::Color;

pub trait Texture {
    fn value(&self, uv: Vec2, point: Vec3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: Vec2, _point: Vec3) -> Color {
        self.albedo
    }
}
