from . import dependencies
from . import utils
from . import tasks
from . import compileResourcePack
import importlib


class CreateDirs(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Dependencies/")
        utils.createDir(self.project_path + "Output/MacOS/")


class InstallGlfw(tasks.Task):
    def execute(self):
        dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.MACOS.zip", self.project_path + "Dependencies/glfw-3.3.7.bin.MACOS/", "glfw")


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
    def checkForDependencies(self):
        self.requireCommand("cmake")

    def execute(self):
        dependencies.installDependency("https://github.com/sago007/PlatformFolders/archive/refs/tags/4.2.0.zip", self.project_path + "Dependencies/PlatformFolders-4.2.0/", "platform folders")


class InstallPerlinNoise(tasks.Task):
    def execute(self):
        dependencies.installDependency("https://github.com/Reputeless/PerlinNoise/archive/refs/tags/v3.0.0.zip", self.project_path + "Dependencies/PerlinNoise-3.0.0/", "perlin noise")


class CompileResourcePack(tasks.Task):
    def execute(self):
        compileResourcePack.compileResourcePack(self.project_path + "Resources", self.project_path + "Build/Resources/")


class BuildClient(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("xcodebuild")
        self.requireCommand("sysctl")

    def execute(self):
        utils.system(f"xcodebuild build -quiet -project {self.project_path}Terralistic.xcodeproj -scheme Terralistic archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {self.project_path}Terralistic.xcarchive")


class BuildServer(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("xcodebuild")
        self.requireCommand("sysctl")

    def execute(self):
        utils.system(f"xcodebuild build -quiet -project {self.project_path}Terralistic.xcodeproj -scheme Terralistic-server archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {self.project_path}Terralistic-server.xcarchive")


class UnpackClient(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("xcodebuild")

    def execute(self):
        utils.remove(self.project_path + "Output/MacOS/Terralistic.app/")
        utils.system(f"xcodebuild -exportArchive -quiet -archivePath {self.project_path}Terralistic.xcarchive -exportPath {self.project_path}Terralistic.app/ -exportOptionsPlist {self.project_path}Terralistic.xcodeproj/exportOptions.plist")
        utils.copy(f"{self.project_path}Terralistic.app/Terralistic.app/", f"{self.project_path}Output/MacOS/Terralistic.app/")
        utils.remove(self.project_path + "Terralistic.xcarchive/")
        utils.remove(self.project_path + "Terralistic.app/")


class UnpackServer(tasks.Task):
    def checkForDependencies(self):
        self.requireCommand("xcodebuild")

    def execute(self):
        utils.remove(self.project_path + "Output/MacOS/Terralistic-server.app/")
        utils.system(f"xcodebuild -exportArchive -quiet -archivePath {self.project_path}Terralistic-server.xcarchive -exportPath {self.project_path}Terralistic-server.app/ -exportOptionsPlist {self.project_path}Terralistic.xcodeproj/exportOptions.plist")
        utils.copy(f"{self.project_path}Terralistic-server.app/Terralistic-server.app/", f"{self.project_path}Output/MacOS/Terralistic-server.app/")
        utils.remove(self.project_path + "Terralistic-server.xcarchive/")
        utils.remove(self.project_path + "Terralistic-server.app/")


def buildForMacOS(project_path: str, arg: str):
    task_manager = tasks.TaskManager()

    task_manager.registerTask(CreateDirs(project_path))
    task_manager.registerTask(InstallGlfw(project_path))
    task_manager.registerTask(GenerateGlad(project_path))
    task_manager.registerTask(InstallPlatformFolders(project_path))
    task_manager.registerTask(InstallPerlinNoise(project_path))
    task_manager.registerTask(CompileResourcePack(project_path))
    if arg != "nobuild":
        task_manager.registerTask(BuildClient(project_path))
        task_manager.registerTask(BuildServer(project_path))
        task_manager.registerTask(UnpackClient(project_path))
        task_manager.registerTask(UnpackServer(project_path))

    task_manager.run()
