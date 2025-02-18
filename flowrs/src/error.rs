use pyo3::prelude::*;
use thiserror::Error;

/// Custom error type for workflow execution.
#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Circular dependency detected")]
    CircularDependency,
    // You can add more error types as needed.
}

impl std::convert::From<WorkflowError> for PyErr {
    fn from(err: WorkflowError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
    }
}
