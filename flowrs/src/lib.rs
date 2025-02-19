mod task;
mod scheduler;

use pyo3::prelude::*;
use scheduler::Scheduler;
use task::Task;
use pyo3::exceptions::PyTypeError;

/// The main Workflow struct exposed to Python.
/// It holds a scheduler that can have tasks added to it.
#[pyclass]
pub struct Workflow {
    #[pyo3(get)]
    name: String,
    scheduler: Scheduler,
}

#[pymethods]
impl Workflow {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        Ok(Workflow {
            name: name,
            scheduler: Scheduler::new()
        })
    }

    /// Add a task to the scheduler.
    pub fn add_task(&mut self, name: String, func: Py<PyAny>, depends_on: Option<Vec<PyObject>>) -> PyResult<()> {
        Python::with_gil(|py| {
            let callable = func.as_ref(py);
            if !callable.is_callable(){
                return Err(PyTypeError::new_err("The provided task is not callable."))
            }
            Ok(())
        })?;

        return self.scheduler.add_task(
            Task::new(name, func, depends_on)
        );
    }

    /// Run all scheduled tasks.
    pub fn run(&self, py: Python) -> PyResult<()> {
        self.scheduler.run(py, &self.name)
    }
}

/// Module initialization
#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Workflow>()?;
    Ok(())
}
