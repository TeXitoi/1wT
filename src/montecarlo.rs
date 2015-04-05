// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Monte Carlo modification of a problem

use prob::Prob;
use rand;

/// Returns a random solution
pub fn montecarlo<R: rand::Rng>(p: &Prob, rng: &mut R) -> Prob {
    let mut res = p.clone();
    rng.shuffle(&mut res.jobs);
    res
}

#[cfg(test)]
mod test {
    use prob::Prob;
    use montecarlo;
    use rand;

    #[test]
    fn montecarlo() {
        for n in 0..100 {
            let mut p = Prob::new_rnd(n);
            let rng: &mut rand::XorShiftRng =
                &mut rand::SeedableRng::from_seed([1, 2, 3, 5]);
            let mut s = montecarlo::montecarlo(&p, rng);
            assert!(p.jobs.len() == s.jobs.len());
            p.jobs.sort();
            s.jobs.sort();
            assert!(p == s);
        }
    }
}
