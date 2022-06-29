import dependencies
import utils


def buildForMacOS(project_path):
    utils.createDir(project_path + "Dependencies/")

    dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.MACOS.zip", project_path + "Dependencies/glfw-3.3.7.bin.MACOS/", "glfw")

    utils.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {project_path}Terralistic.xcarchive")
    utils.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic-server archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {project_path}Terralistic-server.xcarchive")

    utils.createDir(project_path + "Output/MacOS/")

    utils.remove(project_path + "Output/MacOS/Terralistic.app/")
    utils.system(f"xcodebuild -exportArchive -quiet -archivePath {project_path}Terralistic.xcarchive -exportPath {project_path}Terralistic.app/ -exportOptionsPlist {project_path}Terralistic.xcodeproj/exportOptions.plist")
    utils.copy(f"{project_path}Terralistic.app/Terralistic.app/", f"{project_path}Output/MacOS/Terralistic.app/")

    utils.remove(project_path + "Output/MacOS/Terralistic-server.app/")
    utils.system(f"xcodebuild -exportArchive -quiet -archivePath {project_path}Terralistic-server.xcarchive -exportPath {project_path}Terralistic-server.app/ -exportOptionsPlist {project_path}Terralistic.xcodeproj/exportOptions.plist")
    utils.copy(f"{project_path}Terralistic-server.app/Terralistic-server.app/", f"{project_path}Output/MacOS/Terralistic-server.app/")

    utils.remove(project_path + "Terralistic.xcarchive/")
    utils.remove(project_path + "Terralistic-server.xcarchive/")

    utils.remove(project_path + "Terralistic.app/")
    utils.remove(project_path + "Terralistic-server.app/")