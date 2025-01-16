import unittest
from flowrs.functions import create_workflow

class TestFunctions(unittest.TestCase):
    def test_create_workflow(self):
        """Test Workflow creation using `create_workflow`."""
        workflow = create_workflow("dummy_workflow")

        self.assertEqual(workflow.name, "dummy_workflow")

if __name__ == "__main__":
    unittest.main()
