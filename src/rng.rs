use std::ops::Range;

pub fn random_range(range: &Range<f32>) -> f32 {
    fastrand::f32() * (range.end - range.start) + range.start
}
