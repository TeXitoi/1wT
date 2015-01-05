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
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Job {
    /// processing time
    pub p: u32,
    /// due date
    pub d: u32,
    /// weigth
    pub w: u32
}

/// The 1||ΣwT problem description
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Prob {
    /// The list of the jobs
    pub jobs: Vec<Job>
}
impl Prob {
    /// Create an empty problem
    pub fn new() -> Prob {
        Prob { jobs: vec![] }
    }

    /// Create randomly a problem with `n` jobs.
    pub fn new_rnd(n: u32) -> Prob {
        if n == 0 { return Prob::new(); }
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

    /// Add a job at the end of the list of the jobs
    pub fn add(&mut self, p: u32, d: u32, w: u32) -> &mut Prob {
        self.jobs.push(Job { p: p, d: d, w: w });
        self
    }

    /// Evaluate the total weighted tardiness according to the order
    /// of the jobs.
    pub fn eval(&self) -> u32 {
        let mut wt = 0;
        let mut t = 0;
        for &Job { p, d, w } in self.jobs.iter() {
            t += p;
            wt += if d < t { w * (t - d) } else { 0 };
        }
        wt
    }
}

#[cfg(test)]
mod test {
    use prob::Prob;

    #[test]
    fn new_is_empty() {
        assert!(Prob::new().jobs.len() == 0);
    }

    #[test]
    fn new_rnd() {
        for n in range(0, 100) {
            assert!(Prob::new_rnd(n).jobs.len() == n as uint);
        }
    }

    #[test]
    fn eval() {
        let mut prob = Prob::new();
        assert!(prob.eval() == 0);
        prob.add(0, 0, 0);
        assert!(prob.eval() == 0);
        prob.add(5, 5, 1);
        assert!(prob.eval() == 0);
        prob.add(5, 5, 1);
        assert!(prob.eval() == 5);
        prob.add(5, 10, 2);
        assert!(prob.eval() == 15);
        prob.add(10, 100, 2).add(5, 100, 42);
        assert!(prob.eval() == 15);
    }
}
