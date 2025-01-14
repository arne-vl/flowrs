use pyo3::prelude::*;
use pyo3::types::PyFunction;
use chrono::Local;

mod functions;

#[pymodule]
fn flowrs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Workflow>()?;

    m.add_submodule(functions::make_module(py)?)?;

    Ok(())
}

/*  --------------------- Module Implementation --------------------- */

#[pyclass]
pub struct Workflow {
    #[pyo3(get)]
    name: String,

    tasks: Vec<(String, Py<PyFunction>)>,
}

#[pymethods]
impl Workflow {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        Ok(Workflow { name, tasks: Vec::new() })
    }

    pub fn add_task(&mut self, name: String, py_func: Py<PyFunction>) {
        self.tasks.push((name, py_func));
    }

    pub fn run(&self, py: Python) -> PyResult<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] Starting Workflow: {}", timestamp, self.name);

        for (name, py_func) in &self.tasks {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[{}] - Running task: {}", timestamp, name);

            let func = py_func.as_ref(py);
            func.call0()?;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] Finished Workflow: {}", timestamp, self.name);
        Ok(())
    }
}
