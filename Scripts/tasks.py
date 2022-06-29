import abc


class Task(abc.ABC):
    def __init__(self):
        pass

    @abc.abstractmethod
    def checkForDependencies(self):
        pass

    @abc.abstractmethod
    def build(self):
        pass


class TaskManager:
    def __init__(self):
        self.tasks = []

    def registerTask(self, task: Task):
        self.tasks.append(task)
        
    def run(self):
        for task in self.tasks:
            task.checkForDependencies()

        for task in self.tasks:
            task.build()
