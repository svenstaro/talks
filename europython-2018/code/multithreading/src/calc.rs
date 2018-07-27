use rng::rng_maker;
use rand::Rng;
use rayon::prelude::*;

fn in_circle(x: f64, y: f64) -> bool {
    (x * x + y * y).sqrt() <= 1.0
}

pub fn calc(iterations: u64) -> f64 {
    let hits = (1..iterations)
        .into_par_iter()
        .map_with(rng_maker(), |rng, _| {
            if in_circle(rng.gen(), rng.gen()) {
                1
            } else {
                0
            }
        })
        .sum::<u64>();
    let pi = 4.0 * (hits as f64 / iterations as f64);
    pi
}
