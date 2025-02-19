use pyo3::prelude::*;

/// Represents a task in the workflow.
pub struct Task {
    /// The Python callable to execute.
    pub func: PyObject,
    /// Optional list of dependencies (other Python callables).
    pub depends_on: Option<Vec<PyObject>>,
}

impl Task {
    /// Create a new Task.
    pub fn new(func: PyObject, depends_on: Option<Vec<PyObject>>) -> Self {
        Task { func, depends_on }
    }

    /// Executes the task.
    ///
    /// Acquires the GIL and calls the stored Python function.
    pub fn execute(&self, py: Python) -> PyResult<()> {
        let callable = self.func.as_ref(py);
        callable.call0()?;
        Ok(())
    }
}
