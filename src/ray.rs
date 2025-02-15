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

    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    pub fn color(&self) -> Color {
        if self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::green();
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::white().lerp(&Color::blue(), t).unwrap()
    }
}
