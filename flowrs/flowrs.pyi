class Workflow:
  """
  Class representing a Workflow.
  """
  name:str

  def __init__(self, name: str) -> 'Workflow': ...

  def add_task(self, name: str, fn: callable) -> None:
    """
    Adds a task to the Workflow.
    """
    ...
