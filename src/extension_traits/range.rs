use std::ops::Range;

pub trait RangeExt {
    fn merge(&self, other: &Self) -> Self;
}

impl RangeExt for Range<f32> {
    fn merge(&self, other: &Self) -> Self {
        self.start.min(other.start)..self.end.max(other.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let range1 = 0.0..10.0;
        let range2 = 5.0..15.0;
        let merged = range1.merge(&range2);
        assert_eq!(merged, 0.0..15.0);
    }

    #[test]
    fn test_no_overlap() {
        let range1 = 0.0..10.0;
        let range2 = 15.0..20.0;
        let merged = range1.merge(&range2);
        assert_eq!(merged, 0.0..20.0);
    }

    #[test]
    fn test_contained() {
        let range1 = 0.0..15.0;
        let range2 = 5.0..10.0;
        let merged = range1.merge(&range2);
        assert_eq!(merged, 0.0..15.0);
    }
}
