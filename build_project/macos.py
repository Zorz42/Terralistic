from . import utils
from . import tasks
from . import compile_resource_pack
import importlib


class CreateDirs(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Output/MacOS/")

class CompileResourcePack(tasks.Task):
    def execute(self):
        compile_resource_pack.compile_resource_pack(self.project_path + "Resources", self.project_path + "Build/Resources/")


class BuildClient(tasks.Task):
    def check_for_dependencies(self):
        self.requireCommand("cargo-bundle")

    def execute(self):
        utils.system(f"cd {self.project_path} && cargo-bundle bundle --release --bin Terralistic")


class BuildServer(tasks.Task):
    def check_for_dependencies(self):
        self.requireCommand("cargo-bundle")

    def execute(self):
        utils.system(f"cd {self.project_path} && cargo-bundle bundle --release --bin TerralisticServer")


class UnpackClient(tasks.Task):
    def execute(self):
        utils.remove(self.project_path + "Output/MacOS/Terralistic.app/")
        utils.copy(f"{self.project_path}target/release/bundle/osx/Terralistic.app/", f"{self.project_path}Output/MacOS/Terralistic.app/")

class UnpackServer(tasks.Task):
    def execute(self):
        utils.remove(self.project_path + "Output/MacOS/TerralisticServer.app/")
        utils.copy(f"{self.project_path}target/release/bundle/osx/TerralisticServer.app/", f"{self.project_path}Output/MacOS/TerralisticServer.app/")


def build_for_macos(project_path: str, arg: str):
    task_manager = tasks.TaskManager()

    task_manager.registerTask(CreateDirs(project_path))
    task_manager.registerTask(CompileResourcePack(project_path))
    if arg != "nobuild":
        task_manager.registerTask(BuildClient(project_path))
        task_manager.registerTask(BuildServer(project_path))
        task_manager.registerTask(UnpackClient(project_path))
        task_manager.registerTask(UnpackServer(project_path))

    task_manager.run()
