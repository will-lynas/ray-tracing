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

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> Option<f64> {
        let oc = center - self.origin;
        let a = self.direction.length_squared();
        let h = self.direction.dot(&oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((h - discriminant.sqrt()) / a)
        }
    }

    pub fn color(&self) -> Color {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;

        if let Some(t) = self.hit_sphere(center, radius) {
            let n = (self.at(t) - center).unit_vector();
            return Color::from_unit_vector(n).unwrap();
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::white().lerp(&Color::blue(), t).unwrap()
    }
}
