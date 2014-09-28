use std::rand;
use std::rand::distributions::{Sample, Range};

pub struct Job {
    /// processing time
    pub p: u32,
    // due date
    pub d: u32,
    // weigth
    pub w: u32
}

pub struct Prob {
    pub jobs: Vec<Job>
}

pub fn create_prob(n: u32) -> Prob {
    let rng: &mut rand::XorShiftRng =
        &mut rand::SeedableRng::from_seed([n, 2 * n, 3 * n, 5 * n]);
    let mut p = Range::new(1u32, 100);
    let mut d = Range::new(1, 100 * n / 3);
    let mut w = Range::new(1u32, 10);
    Prob {
        jobs: range(0, n).map(|_| Job {
            p: p.sample(rng), d: d.sample(rng), w: w.sample(rng)
        }).collect()
    }
}
