#![feature(use_extern_macros, specialization)]

extern crate pyo3;
extern crate rand;
use pyo3::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[pymodinit]
fn simple_example(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "calc")]
    fn calc_py(iterations: u64) -> PyResult<f64> {
        let out = calc(iterations);
        Ok(out)
    }

    Ok(())
}

fn in_circle(x: f64, y: f64) -> bool {
    (x * x + y * y).sqrt() <= 1.0
}

fn calc(iterations: u64) -> f64 {
    let mut rng = SmallRng::from_rng(rand::thread_rng()).unwrap();

    let hits = (1..iterations)
        .map(|_| {
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
