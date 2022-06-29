import abc
import shutil


class DependencyError(Exception):
    pass


class Task(abc.ABC):
    def __init__(self, project_path):
        self.required_commands = []
        self.project_path = project_path

    def checkForDependencies(self):
        pass

    @abc.abstractmethod
    def execute(self):
        pass

    def requireCommand(self, command):
        self.required_commands.append(command)


class TaskManager:
    def __init__(self):
        self.tasks = []

    def registerTask(self, task: Task):
        self.tasks.append(task)
        
    def run(self):
        required_commands = set()
        print("Checking dependencies...")
        for task in self.tasks:
            task.checkForDependencies()
            for command in task.required_commands:
                required_commands.add(command)

        for command in required_commands:
            if shutil.which(command) is None:
                raise DependencyError(f"Command {command} is not present on this system")

        for i, task in enumerate(self.tasks):
            print(f"[{i + 1}/{len(self.tasks)}] Executing step {task.__class__.__name__}")
            task.execute()
