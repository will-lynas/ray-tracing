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
