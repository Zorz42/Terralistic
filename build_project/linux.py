from . import dependencies
from . import utils
from . import tasks
from . import compileResourcePack
import importlib
import multiprocessing


class CreateDirs(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Dependencies/")
        utils.createDir(self.project_path + "Output/Linux/")


class InstallZlib(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("make")
        self.requireCommand("cmake")

    def execute(self):
        dependencies.installDependency("https://github.com/madler/zlib/archive/refs/heads/master.zip", self.project_path + "Dependencies/zlib-master/", "zlib", f"cd {self.project_path}Dependencies/zlib-master/ && cmake -DCMAKE_INSTALL_PREFIX=. . && cmake --build . --config Release --target install -j{multiprocessing.cpu_count()}")


class InstallGlfw(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("make")
        self.requireCommand("cmake")

    def execute(self):
        dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.zip", self.project_path + "Dependencies/glfw-3.3.7/", "glfw", f"cd {self.project_path}Dependencies/glfw-3.3.7/ && cmake -B build && cd build && make")


class InstallPatch(tasks.Task):
    def execute(self):
        dependencies.installDependency("https://netix.dl.sourceforge.net/project/gnuwin32/patch/2.5.9-7/patch-2.5.9-7-bin.zip", self.project_path + "Dependencies/patch-2.5.9-7-bin/", "patch", create_dir=True)


class GenerateGlad(tasks.Task):
    def checkForDependencies(self):
        glad_spec = importlib.util.find_spec("glad")
        if glad_spec is None:
            raise Exception("Glad needs to be installed as a python package, you can install it via pip")

    def execute(self):
        if not utils.exists(self.project_path + "Dependencies/glad/"):
            utils.system(f"python3 -m glad --profile compatibility --out-path \"{self.project_path}Dependencies/glad/\" --api gl=4.6 --generator c")
        else:
            print("Glad already generated")


class InstallPlatformFolders(tasks.Task):
    def execute(self):
        dependencies.installDependency("https://github.com/sago007/PlatformFolders/archive/refs/tags/4.2.0.zip", self.project_path + "Dependencies/PlatformFolders-4.2.0/", "platform folders")


class InstallPerlinNoise(tasks.Task):
    def execute(self):
        dependencies.installDependency("https://github.com/Reputeless/PerlinNoise/archive/refs/tags/v3.0.0.zip", self.project_path + "Dependencies/PerlinNoise-3.0.0/", "perlin noise")


class CompileResourcePack(tasks.Task):
    def execute(self):
        compileResourcePack.compileResourcePack(self.project_path + "Resources/", self.project_path + "cmake-build-debug/Resources/")
        compileResourcePack.compileResourcePack(self.project_path + "Resources/", self.project_path + "Build/Resources/")


class Build(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("make")
        self.requireCommand("cmake")

    def execute(self):
        utils.system(f"cd {self.project_path}Build/ && cmake .. && make -j{multiprocessing.cpu_count()}")


class UnpackClient(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Output/Linux/Terralistic")
        if utils.exists(self.project_path + "Output/Linux/Terralistic/"):
            utils.remove(self.project_path + "Output/Linux/Terralistic/")
        utils.copy(self.project_path + "Build/Resources/", self.project_path + "Output/Linux/Terralistic/Resources/")
        utils.copy(self.project_path + "Build/Terralistic", self.project_path + "Output/Linux/Terralistic/Terralistic")


class UnpackServer(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Output/Linux/Terralistic-server")
        if utils.exists(self.project_path + "Output/Linux/Terralistic-server/"):
            utils.remove(self.project_path + "Output/Linux/Terralistic-server/")
        utils.copy(self.project_path + "Build/Resources/", self.project_path + "Output/Linux/Terralistic-server/Resources/")
        utils.copy(self.project_path + "Build/Terralistic-server", self.project_path + "Output/Linux/Terralistic-server/Terralistic-server")


def buildForLinux(project_path, arg):
    task_manager = tasks.TaskManager()

    task_manager.registerTask(CreateDirs(project_path))
    task_manager.registerTask(InstallZlib(project_path))
    task_manager.registerTask(InstallGlfw(project_path))
    task_manager.registerTask(GenerateGlad(project_path))
    task_manager.registerTask(InstallPatch(project_path))
    task_manager.registerTask(InstallPlatformFolders(project_path))
    task_manager.registerTask(InstallPerlinNoise(project_path))
    task_manager.registerTask(CompileResourcePack(project_path))

    if arg != "nobuild":
        task_manager.registerTask(Build(project_path))
        task_manager.registerTask(UnpackClient(project_path))
        task_manager.registerTask(UnpackServer(project_path))

    task_manager.run()

    if arg == "run":
        utils.system(project_path + "Output/Linux/Terralistic/Terralistic")
