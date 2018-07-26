use pyo3::prelude::*;

#[pyclass]
pub struct EuroPythonClass {
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
