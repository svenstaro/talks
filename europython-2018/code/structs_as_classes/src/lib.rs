#![feature(use_extern_macros, specialization, extern_prelude)]
extern crate pyo3;
use pyo3::prelude::*;

mod our_class;
use our_class::EuroPythonClass;

#[pymodinit]
fn rust_structs(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EuroPythonClass>()?;
    Ok(())
}
