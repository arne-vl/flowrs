use pyo3::prelude::*;

/// Represents a task in the workflow.
pub struct Task {
    pub name: String,
    func: PyObject,
    depends_on: Option<Vec<PyObject>>,
}

impl Task {
    /// Create a new Task.
    pub fn new(name: String, func: PyObject, depends_on: Option<Vec<PyObject>>) -> Self {
        Task { name, func, depends_on }
    }

    /// Executes the task.
    pub fn execute(&self, py: Python) -> PyResult<()> {
        let callable = self.func.as_ref(py);
        callable.call0()?;
        Ok(())
    }
}
