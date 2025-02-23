pub trait Itertools: Iterator {
    fn cartesian_product<T>(self, other: T) -> CartesianProduct<Self, T::IntoIter>
    where
        Self: Sized,
        Self::Item: Clone,
        T: IntoIterator,
        T::IntoIter: Clone,
    {
        CartesianProduct::new(self, other.into_iter())
    }
}

impl<T> Itertools for T where T: Iterator {}

pub struct CartesianProduct<I, J>
where
    I: Iterator,
{
    a: I,
    #[allow(clippy::option_option)]
    a_cur: Option<Option<I::Item>>,
    b: J,
    b_orig: J,
}

impl<I, J> CartesianProduct<I, J>
where
    I: Iterator,
    J: Clone,
{
    pub fn new(a: I, b: J) -> Self {
        CartesianProduct {
            a,
            a_cur: None,
            b: b.clone(),
            b_orig: b,
        }
    }
}

impl<I, J> Iterator for CartesianProduct<I, J>
where
    I: Iterator,
    I::Item: Clone,
    J: Iterator + Clone,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match &self.a_cur {
            // Finished
            Some(None) => None,
            // Not started
            None => {
                self.a_cur = Some(self.a.next());
                self.next()
            }
            // Started
            Some(Some(a_cur)) => match self.b.next() {
                None => {
                    self.a_cur = Some(self.a.next());
                    self.b = self.b_orig.clone();
                    self.next()
                }
                Some(b_cur) => Some((a_cur.clone(), b_cur)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_product() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = a.into_iter().cartesian_product(b);
        assert_eq!(
            c.collect::<Vec<_>>(),
            vec![
                (1, 4),
                (1, 5),
                (1, 6),
                (2, 4),
                (2, 5),
                (2, 6),
                (3, 4),
                (3, 5),
                (3, 6)
            ]
        );
    }

    #[test]
    fn test_cartesian_product_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        let c = a.into_iter().cartesian_product(b);
        assert_eq!(c.collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_cartesian_product_empty_a() {
        let a: Vec<i32> = vec![];
        let b = vec![4, 5, 6];
        let c = a.into_iter().cartesian_product(b);
        assert_eq!(c.collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_cartesian_product_empty_b() {
        let a = vec![1, 2, 3];
        let b: Vec<i32> = vec![];
        let c = a.into_iter().cartesian_product(b);
        assert_eq!(c.collect::<Vec<_>>(), vec![]);
    }
}
