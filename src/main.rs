// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![license = "WTFPLv2"]
#![deny(missing_doc)]
#![deny(warnings)]

#![feature(phase)]

//! This program generate the problem.

#[phase(plugin, link)]
extern crate mdo;

pub mod prob;

#[cfg(not(test))]
fn main () {
    use mdo::option::bind;

    let args = std::os::args();
    let n: u32 = mdo! {
        s <- args[].get(1);
        ret std::from_str::from_str(s.as_slice())
    }.expect("first arg must be the number of jobs");

    let prob = prob::create_prob(n);
    for j in prob.jobs.iter() {
        println!("{} {} {}", j.p, j.d, j.w);
    }
}
