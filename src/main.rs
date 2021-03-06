// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(collections)]

//! This program generate the problem.

#[macro_use]
extern crate mdo;
extern crate time;
extern crate rand;

use prob::Prob;

pub mod prob;
pub mod montecarlo;

/// Chrono
pub struct Chrono<T> {
    /// the result
    pub res: T,
    /// the ducation of the computation
    pub dur: f64
}

///compute and chronometer
pub fn chrono<T, F: FnMut() -> T>(mut f: F) -> Chrono<T> {
    use time::precise_time_s;
    let start = precise_time_s();
    let res = f();
    let end = precise_time_s();
    Chrono { res: res, dur: end - start }
}

/// find the optimal solution by bruteforce
pub fn solve_optimal(prob: &Prob) -> (u32, usize) {
    let mut s = prob.clone();
    let mut opt_wt = s.eval();
    let mut search_space = 0;
    for (i, j) in std::slice::ElementSwaps::new(s.jobs.len()) {
        search_space += 1;
        s.jobs.swap(i, j);
        let cur_wt = s.eval();
        if cur_wt < opt_wt {
            opt_wt = cur_wt;
            println!("better at {} = {}", search_space, opt_wt);
        }
    }
    (opt_wt, search_space)
}

/// find using montecarlo until a solution of quality lb is found
pub fn solve_mc(prob: &Prob, lb: u32) -> (u32, usize) {
    use rand;

    let mut i = 0;
    let rng: &mut rand::XorShiftRng =
        &mut rand::SeedableRng::from_seed([1, 2, 3, 5]);
    let mut best_wt = prob.eval();
    println!("Initial eval = {}", best_wt);
    loop {
        i += 1;
        let s = montecarlo::montecarlo(prob, rng);
        let cur_wt = s.eval();
        if cur_wt < best_wt {
            best_wt = cur_wt;
            println!("better at {} = {}", i, best_wt);
        }
        if cur_wt <= lb { return (cur_wt, i); }
    }
}

#[cfg(not(test))]
fn main () {
    use mdo::option::bind;

    let n: u32 = mdo! {
        s =<< std::env::args_os().nth(1);
        s =<< s.to_str();
        ret s.parse().ok()
    }.expect("first arg must be the number of jobs");

    let prob = prob::Prob::new_rnd(n);

    let res = chrono(|| solve_optimal(&prob));
    println!("opt = {} in {} iterations ({}s)", res.res.0, res.res.1, res.dur);

    let res = chrono(|| solve_mc(&prob, res.res.0));
    println!("opt = {} in {} iterations ({}s)", res.res.0, res.res.1, res.dur);
}
