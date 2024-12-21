import unittest
from flowrs import Workflow, Task

class TestFlowrs(unittest.TestCase):
    def test_task_initialization(self):
        """Test Task initialization with name and function."""
        def dummy_task():
            pass

        task = Task("dummy_task", dummy_task)
        self.assertEqual(task.get_name(), "dummy_task")

    def test_task_direct_execution_error(self):
        """Test that Task objects cannot be run directly."""
        def dummy_task():
            pass

        task = Task("dummy_task", dummy_task)

        with self.assertRaises(AttributeError):
            task.run()

    def test_workflow_execution(self):
        """Test Workflow executes tasks in order."""
        results = []

        def task_one():
            results.append("task_one")

        def task_two():
            results.append("task_two")

        task1 = Task("task_one", task_one)
        task2 = Task("task_two", task_two)

        workflow = Workflow("example_workflow")
        workflow.add_task(task1)
        workflow.add_task(task2)

        workflow.run()
        self.assertEqual(results, ["task_one", "task_two"])

if __name__ == "__main__":
    unittest.main()
