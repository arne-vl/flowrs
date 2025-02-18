use pyo3::prelude::*;
use crate::task::Task;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crossbeam_queue::SegQueue;
use pyo3::AsPyPointer;

#[derive(Clone)]
pub struct Scheduler {
    // Internal list of tasks.
    // Note: We require Task to be cloneable to share among threads.
    pub tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { tasks: Vec::new() }
    }

    /// Add a new task to the scheduler.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Execute all tasks while resolving dependencies.
    ///
    /// This implementation:
    /// 1. Builds a dependency graph based on Python callable pointers.
    /// 2. Checks that each dependency is found.
    /// 3. Uses a thread-safe queue and atomic counters to schedule tasks in parallel.
    /// 4. Executes tasks using Rayon and Crossbeam.
    ///
    /// # Errors
    ///
    /// Returns a Python exception if a dependency is missing or if a worker thread panics.
    pub fn execute(&self) -> PyResult<()> {
        let n = self.tasks.len();
        let mut in_degree = vec![0; n];
        // children[i] will store indices of tasks that depend on task i.
        let mut children: Vec<Vec<usize>> = vec![vec![]; n];

        // Build a mapping of tasks by their Python function pointer.
        let func_ptrs: Vec<*mut pyo3::ffi::PyObject> = self
            .tasks
            .iter()
            .map(|t| t.func.as_ptr())
            .collect();

        // For each task, resolve dependencies by matching the callable pointers.
        for (i, task) in self.tasks.iter().enumerate() {
            if let Some(deps) = &task.depends_on {
                for dep in deps {
                    let dep_ptr = dep.as_ptr();
                    if let Some(j) = func_ptrs.iter().position(|&p| p == dep_ptr) {
                        in_degree[i] += 1;
                        children[j].push(i);
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            format!("Dependency not found for task {}", i),
                        ));
                    }
                }
            }
        }

        // Prepare for parallel execution.
        // A thread-safe queue holds indices of tasks ready to run.
        let queue = Arc::new(SegQueue::new());
        let dep_counts: Arc<Vec<AtomicUsize>> = Arc::new(
            in_degree
                .into_iter()
                .map(|count| AtomicUsize::new(count))
                .collect(),
        );
        // Wrap tasks in an Arc for shared ownership.
        let tasks = Arc::new(self.tasks.clone());

        // Enqueue tasks that have no dependencies.
        for i in 0..tasks.len() {
            if dep_counts[i].load(Ordering::SeqCst) == 0 {
                queue.push(i);
            }
        }

        // Determine the number of worker threads.
        let num_workers = rayon::current_num_threads();

        // Use crossbeam's scoped threads to process the task queue.
        let _ = crossbeam_utils::thread::scope(|s| {
            for _ in 0..num_workers {
                let queue = Arc::clone(&queue);
                let dep_counts = Arc::clone(&dep_counts);
                let tasks = Arc::clone(&tasks);
                let children = children.clone(); // clone the dependency graph
                s.spawn(move |_| {
                    // Process tasks until the queue is empty.
                    while let Some(task_index) = queue.pop() {
                        // Execute the task, acquiring the GIL for Python calls.
                        Python::with_gil(|py| {
                            if let Err(e) = tasks[task_index].execute(py) {
                                // For now, we simply print the error.
                                e.print(py);
                            }
                        });
                        // Notify dependent tasks.
                        for &child_index in &children[task_index] {
                            let old = dep_counts[child_index].fetch_sub(1, Ordering::SeqCst);
                            if old == 1 {
                                // If this was the last dependency, schedule the child task.
                                queue.push(child_index);
                            }
                        }
                    }
                });
            }
            // Explicitly annotate the Ok type for the scope.
            Ok::<(), ()>(())
        })
        .map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("A worker thread panicked")
        })?;

        Ok(())
    }
}
