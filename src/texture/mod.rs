use std::fmt::Debug;

use glam::{
    Vec2,
    Vec3A as Vec3,
};

use crate::color::Color;

pub trait Texture: Sync + Debug {
    fn value(&self, uv: Vec2, point: Vec3) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }

    pub fn new_from_color(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: Vec2, _point: Vec3) -> Color {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    squares: f32,
}

impl CheckerTexture {
    pub fn new(odd: impl Texture + 'static, even: impl Texture + 'static, squares: f32) -> Self {
        Self {
            odd: Box::new(odd),
            even: Box::new(even),
            squares,
        }
    }

    pub fn new_from_color(odd: Color, even: Color, squares: f32) -> Self {
        Self::new(
            SolidColor::new_from_color(odd),
            SolidColor::new_from_color(even),
            squares,
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: Vec2, point: Vec3) -> Color {
        let u = (uv.x * self.squares).floor() as i32;
        let v = (uv.y * self.squares).floor() as i32;

        if (u + v) % 2 == 0 {
            self.even.value(uv, point)
        } else {
            self.odd.value(uv, point)
        }
    }
}
