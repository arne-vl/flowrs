import unittest
from unittest.mock import patch
from flowrs import run_python_function

class TestFlowrs(unittest.TestCase):
    def test_run_python_function(self):
        def python_func():
            print("Python function called!")

        # Use unittest.mock to capture print statements
        with patch("builtins.print") as mock_print:
            # Call the Rust function that runs the Python function
            run_python_function(python_func)

            # Assert that the Python function was called (i.e., print was invoked)
            mock_print.assert_called_with("Python function called!")

if __name__ == "__main__":
    unittest.main()
