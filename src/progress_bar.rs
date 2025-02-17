pub trait ProgressBar: Iterator + Sized {
    fn progress_count(self, total: u64) -> ProgressBarIterator<Self> {
        ProgressBarIterator::new(self, total)
    }
}

impl<T> ProgressBar for T where T: Iterator {}

const BAR_LENGTH: usize = 100;

pub struct ProgressBarIterator<I> {
    iterator: I,
    total: u64,
    current: u64,
    started: bool,
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
            started: false,
        }
    }

    fn print_progress(&self) {
        let progress = self.current as f64 / self.total as f64;
        let bar_width = (BAR_LENGTH as f64 * progress) as usize;
        print!(
            "\r[{}{}] {}%",
            "=".repeat(bar_width),
            " ".repeat(BAR_LENGTH - bar_width),
            (progress * 100.0).round() as u64
        );
    }

    fn print_start() {
        print!("\x1B[?25l"); // Turn off cursor
    }

    fn print_end() {
        print!("\r"); // Move to the beginning of the line
        print!("\x1B[2K"); // Clear the line
        print!("\x1B[?25h"); // Turn on cursor
    }
}

impl<I> Iterator for ProgressBarIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            Self::print_start();
        }

        if let Some(item) = self.iterator.next() {
            self.print_progress();
            self.current += 1;
            Some(item)
        } else {
            Self::print_end();
            None
        }
    }
}
