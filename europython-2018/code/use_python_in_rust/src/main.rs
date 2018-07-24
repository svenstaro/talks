extern crate pyo3;
use pyo3::prelude::*;
fn main() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sys = py.import("sys")?;
    let version: String = sys.get("version")?.extract()?;
    println!("Hello from Python {}", version);
    Ok(())
}
