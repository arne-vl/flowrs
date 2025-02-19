mod task;
mod scheduler;

use pyo3::prelude::*;
use scheduler::Scheduler;
use task::Task;

/// The main Workflow struct exposed to Python.
/// It holds a scheduler that can have tasks added to it.
#[pyclass]
pub struct Workflow {
    scheduler: Scheduler,
}

#[pymethods]
impl Workflow {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Workflow { scheduler: Scheduler::new() })
    }

    /// Add a task to the scheduler.
    pub fn add_task(&mut self, func: PyObject, depends_on: Option<Vec<PyObject>>) -> PyResult<()> {
        return self.scheduler.add_task(
            Task::new(func, depends_on)
        );
    }

    /// Run all scheduled tasks.
    pub fn run(&self, py: Python) -> PyResult<()> {
        self.scheduler.run(py)
    }
}

/// Module initialization
#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Workflow>()?;
    Ok(())
}
