use crate::vec3::Vec3;
use std::{
    fmt::{self, Display, Formatter},
    ops::Mul,
};

pub const BLACK: Color = Color::unchecked_new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::unchecked_new(1.0, 1.0, 1.0);
pub const RED: Color = Color::unchecked_new(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::unchecked_new(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::unchecked_new(0.0, 0.0, 1.0);
pub const GREY: Color = Color::unchecked_new(0.5, 0.5, 0.5);
pub const PURPLE: Color = Color::unchecked_new(1.0, 0.0, 1.0);

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Option<Self> {
        ((0.0..=1.0).contains(&r) && (0.0..=1.0).contains(&g) && (0.0..=1.0).contains(&b))
            .then_some(Self { r, g, b })
    }

    const fn unchecked_new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Option<Self> {
        (0.0..=1.0).contains(&t).then_some(
            Self::new(
                (1.0 - t) * self.r + t * other.r,
                (1.0 - t) * self.g + t * other.g,
                (1.0 - t) * self.b + t * other.b,
            )
            .unwrap(),
        )
    }

    pub fn average(colors: &[Self]) -> Option<Self> {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        for color in colors {
            r += color.r;
            g += color.g;
            b += color.b;
        }

        Self::new(
            r / colors.len() as f64,
            g / colors.len() as f64,
            b / colors.len() as f64,
        )
    }

    pub fn mul(&self, s: f64) -> Option<Self> {
        (0.0..=1.0)
            .contains(&s)
            .then_some(Self::new(self.r * s, self.g * s, self.b * s).unwrap())
    }

    pub fn from_unit_vector(n: Vec3) -> Option<Self> {
        Self::new(0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let r = (self.r.sqrt() * 255.999) as u8;
        let g = (self.g.sqrt() * 255.999) as u8;
        let b = (self.b.sqrt() * 255.999) as u8;
        write!(f, "{r} {g} {b}")
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b).unwrap()
    }
}
