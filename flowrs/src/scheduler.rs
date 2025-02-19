use crate::task::Task;
use pyo3::prelude::*;
use chrono::Local;
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
    pub fn run(&self, py: Python, workflow_name: &String) -> PyResult<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] Starting Workflow: {}", timestamp, workflow_name);

        for task in &self.tasks {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[{}] - Running task: {}", timestamp, task.name);
            task.execute(py)?;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] Finished Workflow: {}", timestamp, workflow_name);

        Ok(())
    }
}
