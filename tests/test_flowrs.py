import unittest
from flowrs import Workflow

class TestFlowrs(unittest.TestCase):
    def test_workflow_initialization(self):
        """Test Workflow initialization with name and function."""

        workflow = Workflow("dummy_workflow")
        self.assertEqual(workflow.get_name(), "dummy_workflow")

if __name__ == "__main__":
    unittest.main()
