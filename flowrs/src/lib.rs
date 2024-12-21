use pyo3::prelude::*;
use chrono::Local;
use pyo3::types::PyFunction;

#[pyclass]
pub struct Task {
    name: String,
    py_func: Py<PyFunction>
}

#[pymethods]
impl Task {
    #[new]
    pub fn new(name: String, py_func: Py<PyFunction>) -> Self {
        Task { name, py_func }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self, py: Python) -> PyResult<()> {
        let func = self.py_func.as_ref(py);
        func.call0()?;
        Ok(())
    }
}

#[pyclass]
pub struct Workflow {
    name: String,
    tasks: Vec<Py<Task>>
}

#[pymethods]
impl Workflow {
    #[new]
    pub fn new(name: String) -> Self {
        Workflow {
            name,
            tasks: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_task(&mut self, task: Py<Task>) {
        self.tasks.push(task);
    }

    pub fn run(&self, py: Python) -> PyResult<()> {
        for task in &self.tasks {
            let task_ref = task.borrow(py);
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[{}] Running task: {}", timestamp, task_ref.get_name());
            task_ref.execute(py)?;
        }
        Ok(())
    }
}

#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Task>()?;
    m.add_class::<Workflow>()?;
    Ok(())
}
