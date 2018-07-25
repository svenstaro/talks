#![feature(use_extern_macros, specialization)]
extern crate pyo3;
use pyo3::prelude::*;

#[pyclass]
struct EuroPythonClass {
    our_strings: Vec<String>,
    token: PyToken,
}

#[pymethods]
impl EuroPythonClass {
    #[new]
    fn __new__(obj: &PyRawObject,
               our_strings: Vec<String>) -> PyResult<()> {
        obj.init(|token| EuroPythonClass { our_strings, token })
    }

    fn num_strings(&self) -> usize {
        self.our_strings.len()
    }
}

#[pymodinit]
fn rust_structs(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EuroPythonClass>()?;
    Ok(())
}
