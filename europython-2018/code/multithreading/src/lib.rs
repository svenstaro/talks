#![feature(use_extern_macros)]
extern crate pyo3;
extern crate rand;
extern crate rayon;
use pyo3::prelude::*;
mod calc; mod rng;

#[pymodinit]
fn multithreading(_: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "calc")]
    fn calc_py(iterations: u64) -> PyResult<f64> {
        let out = calc::calc(iterations);
        Ok(out)
    }
    Ok(())
}
