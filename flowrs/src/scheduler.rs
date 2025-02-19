use crate::task::Task;
use pyo3::prelude::*;

pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { tasks: Vec::new() }
    }

    /// Adds a task to the scheduler.
    pub fn add_task(&mut self, task: Task) -> PyResult<()> {
        self.tasks.push(task);
        Ok(())
    }

    /// Runs all scheduled tasks.
    pub fn run(&self, py: Python) -> PyResult<()> {
        for task in &self.tasks {
            task.execute(py)?;
        }
        Ok(())
    }
}
