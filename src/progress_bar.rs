pub trait ProgressBarIter: Iterator + Sized {
    fn progress_count(self, total: u64) -> ProgressBarIterator<Self> {
        ProgressBarIterator::new(self, total)
    }
}

impl<T> ProgressBarIter for T where T: Iterator {}

const BAR_LENGTH: usize = 100;
const PRINT_INTERVAL: f64 = 0.0001;

pub struct ProgressBarIterator<I> {
    iterator: I,
    total: u64,
    current: u64,
    started: bool,
    last_progress: f64,
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
            last_progress: -PRINT_INTERVAL * 2.0,
        }
    }

    fn print_progress(&mut self) {
        let progress = self.current as f64 / self.total as f64;

        if progress - self.last_progress > PRINT_INTERVAL {
            let partial_bar_length = (BAR_LENGTH as f64 * progress) as usize;
            print!(
                "\r[{}{}] {:.2}%",
                "=".repeat(partial_bar_length),
                " ".repeat(BAR_LENGTH - partial_bar_length),
                progress * 100.0
            );
            self.last_progress = progress;
        }
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
