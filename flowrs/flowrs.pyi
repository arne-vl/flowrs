from typing import Callable, List, Optional, Any

class Workflow:
    """
    A parallel workflow runner that executes Python tasks in parallel with dependency resolution.

    Methods:
        __init__(): Initializes a new workflow.
        add_task(func, depends_on=None): Adds a task to the workflow.
        run(): Executes the workflow.
    """

    def __init__(self) -> None: ...

    def add_task(
        self,
        func: Callable[..., Any],
        depends_on: Optional[List[Callable[..., Any]]] = None
    ) -> None:
        """
        Adds a task to the workflow.

        Args:
            func: A Python callable representing the task.
            depends_on: An optional list of callables that this task depends on.
        """
        ...

    def run(self) -> None:
        """
        Runs all tasks in the workflow, resolving dependencies.

        Raises:
            RuntimeError: If a scheduling error occurs (e.g., circular dependency).
        """
        ...
