use glam::Vec3A as Vec3;

#[derive(Clone, Copy)]
pub struct TimedRay {
    pub origin: Vec3,
    pub direction: Vec3,
    // Time that the ray was emitted, between 0 and 1
    // Not to be confused with parameterization of the ray through space.
    pub time: f32,
}

impl TimedRay {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
