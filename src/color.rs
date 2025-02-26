use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    ops::Mul,
};

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
pub struct Color(Vec3);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    const fn new_u8(r: u8, g: u8, b: u8) -> Self {
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
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let r = (self.0.x.sqrt() * 255.999) as u8;
        let g = (self.0.y.sqrt() * 255.999) as u8;
        let b = (self.0.z.sqrt() * 255.999) as u8;
        write!(f, "{r} {g} {b}")
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}
