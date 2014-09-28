#![feature(phase)]

#[phase(plugin, link)]
extern crate mdo;

pub mod prob;

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
