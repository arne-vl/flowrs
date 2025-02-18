mod scheduler;
mod task;
mod error;

use pyo3::prelude::*;
use scheduler::Scheduler;
use task::Task;

/// The main Workflow struct exposed to Python.
/// It holds a list of tasks and manages execution.
#[pyclass]
pub struct Workflow {
    scheduler: Scheduler,
}

#[pymethods]
impl Workflow {
    /// Creates a new, empty Workflow.
    #[new]
    pub fn new() -> Self {
        Workflow {
            scheduler: Scheduler::new(),
        }
    }

    /// Adds a task to the workflow.
    ///
    /// Args:
    ///     func: A Python callable representing the task.
    ///     depends_on: An optional list of callables that this task depends on.
    pub fn add_task(&mut self, func: PyObject, depends_on: Option<Vec<PyObject>>) -> PyResult<()> {
        let task = Task::new(func, depends_on);
        self.scheduler.add_task(task);
        Ok(())
    }

    /// Runs all tasks in parallel, resolving dependencies.
    ///
    /// Raises:
    ///     RuntimeError: If there is a circular dependency or other scheduling error.
    pub fn run(&self) -> PyResult<()> {
        self.scheduler.execute()
    }
}

/// Module initialization
#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Workflow>()?;
    Ok(())
}
