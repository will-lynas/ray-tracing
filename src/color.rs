use std::ops::Mul;

use glam::Vec3A as Vec3;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);
pub const GREY: Color = Color::new(0.5, 0.5, 0.5);
pub const PURPLE: Color = Color::new(1.0, 0.0, 1.0);
pub const TURQUOISE: Color = Color::new_u8(175, 238, 238);
pub const LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.0);

#[derive(Debug, Clone, Copy, Default)]
pub struct Color(pub Vec3);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub const fn new_u8(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0.lerp(other.0, t))
    }

    pub fn average(colors: &[Self]) -> Self {
        Self(colors.iter().map(|c| c.0).sum::<Vec3>() / colors.len() as f32)
    }

    pub fn from_unit_vector(n: Vec3) -> Self {
        Self((n + Vec3::ONE) * 0.5)
    }

    pub fn bytes(&self) -> [u8; 3] {
        [
            Self::float_to_u8(self.0.x),
            Self::float_to_u8(self.0.y),
            Self::float_to_u8(self.0.z),
        ]
    }

    fn float_to_u8(f: f32) -> u8 {
        (f.sqrt() * 256.0).clamp(0.0, 256.0) as u8
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl From<&image::Rgb<u8>> for Color {
    fn from(rgb: &image::Rgb<u8>) -> Self {
        Self::new_u8(rgb.0[0], rgb.0[1], rgb.0[2])
    }
}
