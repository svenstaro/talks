#![feature(use_extern_macros)]
extern crate pyo3;
use pyo3::prelude::*;
#[pymodinit]
fn europython(_: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "hello")]
    fn hello(name: String) -> PyResult<()> {
        println!("Hello, {} 💖!", name);
        Ok(())
    }
    Ok(())
}
