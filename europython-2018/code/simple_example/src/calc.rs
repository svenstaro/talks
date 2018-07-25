#![feature(use_extern_macros, specialization)]
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng, thread_rng};

fn in_circle(x: f64, y: f64) -> bool {
    (x * x + y * y).sqrt() <= 1.0
}

pub fn calc(iterations: u64) -> f64 {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let hits = (1..iterations)
        .map(|_| {
            if in_circle(rng.gen(), rng.gen()) { 1 } else { 0 }
        })
        .sum::<u64>();

    let pi = 4.0 * (hits as f64 / iterations as f64);
    pi
}
