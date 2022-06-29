from Scripts import dependencies
from Scripts import utils
from Scripts import tasks


class CreateDirs(tasks.Task):
    def execute(self):
        utils.createDir(self.project_path + "Dependencies/")
        utils.createDir(self.project_path + "Output/MacOS/")


class InstallGlfw(tasks.Task):
    def checkForDependencies(self):
        pass

    def execute(self):
        dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.MACOS.zip", self.project_path + "Dependencies/glfw-3.3.7.bin.MACOS/", "glfw")


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
        utils.system(
            f"xcodebuild build -quiet -project {self.project_path}Terralistic.xcodeproj -scheme Terralistic-server archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {self.project_path}Terralistic-server.xcarchive")


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


def buildForMacOS(project_path):
    task_manager = tasks.TaskManager()

    task_manager.registerTask(CreateDirs(project_path))
    task_manager.registerTask(InstallGlfw(project_path))
    task_manager.registerTask(BuildClient(project_path))
    task_manager.registerTask(BuildServer(project_path))
    task_manager.registerTask(UnpackClient(project_path))
    task_manager.registerTask(UnpackServer(project_path))

    task_manager.run()
