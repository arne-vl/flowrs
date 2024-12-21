# flowrs
Flowrs is a high-performance Python package built in Rust, designed to simplify task orchestration and workflow automation. It combines Rust’s speed and reliability with Python’s accessibility, making it a powerful yet user-friendly solution for managing complex processes.

## Usage
### Defining a Task
```python
from flowrs import Task

def my_task():
	print("Task executed!")

task = Task("example_task", my_task)
```

### Defining a Workflow
```python
from flowrs import Workflow

workflow = Workflow("example_workflow")
workflow.add_task(task)

workflow.run()
```
