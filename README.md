# flowrs
Flowrs is a high-performance Python package built in Rust, designed to simplify task orchestration and workflow automation. It combines Rust’s speed and reliability with Python’s accessibility, making it a powerful yet user-friendly solution for managing complex processes.

<a href="https://pypi.org/project/flowrs/">
	<img alt="Static Badge" src="https://img.shields.io/badge/pip-flowrs-blue">
</a>
<a href="https://pypi.org/project/flowrs/">
	<img alt="PyPI - Version" src="https://img.shields.io/pypi/v/flowrs">
</a>

## Defining and running a Workflow
```python
from flowrs import Workflow

def example_task:
	# implement task
	return

workflow = Workflow("example_workflow")
workflow.add_task("example_task", task)

workflow.run()
```
