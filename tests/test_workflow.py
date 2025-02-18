import unittest
from flowrs import Workflow
from typing import List

class TestWorkflow(unittest.TestCase):
    def test_workflow_initialization(self):
        """Test Workflow initialization with name and function."""

        workflow = Workflow("dummy_workflow")
        self.assertEqual(workflow.name, "dummy_workflow")

    def test_workflow_sequential(self):
        """Test Workflow sequential execution."""
        number_list: List[int] = []

        def task_one():
            number_list.append(1)

        def task_two():
            number_list.append(2)

        workflow = Workflow("dummy_workflow")

        workflow.add_task("Task One", task_one)
        workflow.add_task("Task Two", task_two)

        workflow.run()

        self.assertEqual(number_list, [1, 2])

    def test_add_task_non_callable(self):
        """Test adding non callables to a Workflow."""

        workflow = Workflow("dummy_workflow")

        with self.assertRaises(TypeError):
            workflow.add_task("dummy_task", "non_callable")

        with self.assertRaises(TypeError):
            workflow.add_task("dummy_task", 100)

if __name__ == "__main__":
    unittest.main()
