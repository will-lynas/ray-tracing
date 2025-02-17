pub trait ProgressBar: Iterator + Sized {
    fn progress_count(self, total: u64) -> ProgressBarIterator<Self> {
        ProgressBarIterator::new(self, total)
    }
}

impl<T> ProgressBar for T where T: Iterator {}

pub struct ProgressBarIterator<I> {
    iterator: I,
    total: u64,
    current: u64,
}

impl<I> ProgressBarIterator<I>
where
    I: Iterator,
{
    pub fn new(iterator: I, total: u64) -> Self {
        Self {
            iterator,
            total,
            current: 0,
        }
    }
}

impl<I> Iterator for ProgressBarIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iterator.next() {
            print!("\rProgress: {}/{}", self.current, self.total);
            self.current += 1;
            Some(item)
        } else {
            print!("\r");
            None
        }
    }
}
