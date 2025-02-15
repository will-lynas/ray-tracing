use crate::vec3::Vec3;
use std::fmt::{self, Display, Formatter};

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

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0).unwrap()
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0).unwrap()
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0).unwrap()
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

    pub fn from_unit_vector(n: Vec3) -> Option<Self> {
        Self::new(0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.r * 255.999,
            self.g * 255.999,
            self.b * 255.999
        )
    }
}
