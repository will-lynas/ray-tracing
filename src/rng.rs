use std::{
    cell::RefCell,
    ops::Range,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

struct Rng {
    state: u32,
}

impl Rng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        self.state
    }
}

thread_local! {
    static THREAD_RNG: RefCell<Rng> = RefCell::new(Rng::new(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
    ));
}

pub struct ThreadRng;

impl ThreadRng {
    pub fn next_u32() -> u32 {
        THREAD_RNG.with(|rng| rng.borrow_mut().next_u32())
    }

    pub fn random() -> f64 {
        f64::from(Self::next_u32()) / f64::from(u32::MAX)
    }

    pub fn random_range(range: Range<f64>) -> f64 {
        Self::random() * (range.end - range.start) + range.start
    }
}
