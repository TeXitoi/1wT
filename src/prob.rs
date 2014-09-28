// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Description of the problem

use std::rand;
use std::rand::distributions::{Sample, Range};

/// A Job
pub struct Job {
    /// processing time
    pub p: u32,
    /// due date
    pub d: u32,
    /// weigth
    pub w: u32
}

/// The 1||ΣwT problem description
pub struct Prob {
    /// The list of the jobs
    pub jobs: Vec<Job>
}

/// Create a problem with `n` jobs.
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
