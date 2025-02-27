use std::ops::Range;

pub trait RangeExt {
    fn merge(&self, other: &Self) -> Self;
}

impl RangeExt for Range<f32> {
    fn merge(&self, other: &Self) -> Self {
        self.start.min(other.start)..self.end.max(other.end)
    }
}
