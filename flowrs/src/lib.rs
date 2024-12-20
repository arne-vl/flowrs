use pyo3::prelude::*;
use pyo3::types::PyFunction;

#[pyfunction]
fn run_python_function(_py: Python, py_func: &PyFunction) -> PyResult<()> {
    py_func.call0()?;

    Ok(())
}

#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_python_function, m)?)?;

    Ok(())
}
