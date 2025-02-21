# flowrs
Workflow automation with the power of Rust. Let your automations blossom!

<a href="https://pypi.org/project/flowrs/">
	<img alt="Static Badge" src="https://img.shields.io/badge/pip-flowrs-blue">
</a>
<a href="https://pypi.org/project/flowrs/">
	<img alt="PyPI - Version" src="https://img.shields.io/pypi/v/flowrs?color=green">
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
