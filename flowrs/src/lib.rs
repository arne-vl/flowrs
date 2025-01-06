use pyo3::prelude::*;
use pyo3::types::PyFunction;
use chrono::Local;

#[pyclass]
pub struct Workflow {
    name: String,
    tasks: Vec<(String, Py<PyFunction>)>
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

    pub fn add_task(&mut self, name: String, py_func: Py<PyFunction>) {
        self.tasks.push((name, py_func));
    }

    pub fn run(&self, py: Python) -> PyResult<()> {
        for (name, py_func) in &self.tasks {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[{}] Running task: {}", timestamp, name);

            let func = py_func.as_ref(py);
            func.call0()?;
        }
        Ok(())
    }
}

#[pymodule]
fn flowrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Workflow>()?;
    Ok(())
}
