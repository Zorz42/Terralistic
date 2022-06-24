#!/usr/bin/env python3

import sys
import Scripts.utils as utils
import Scripts.dependencies as dependencies

project_path = utils.getDir(__file__)


def buildForMacOS():
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
    
    
def buildForWindows():
    utils.createDir(project_path + "Dependencies/")

    dependencies.installDependency("https://github.com/madler/zlib/archive/refs/heads/master.zip", project_path + "Dependencies/zlib-master/", "zlib", f"\"\"C:/Program Files/Microsoft Visual Studio/2022/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Dependencies/zlib-master/ && cmake -DCMAKE_INSTALL_PREFIX=. -G \"Visual Studio 17 2022\" -A Win32 . && cmake --build . --config Release --target install\"")

    dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.WIN32.zip", project_path + "Dependencies/glfw-3.3.7.bin.WIN32/", "glfw")

    utils.createDir(project_path + "Build/")

    utils.system(f"\"\"C:/Program Files/Microsoft Visual Studio/2022/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Build/ && cmake -DCMAKE_BUILD_TYPE=Release -G \"CodeBlocks - NMake Makefiles\" .. && cmake --build .\"")

    utils.createDir(project_path + "Output/Windows/Terralistic/")
    utils.copy(f"{project_path}Build/Terralistic.exe", f"{project_path}Output/Windows/Terralistic/Terralistic.exe")

    utils.createDir(project_path + "Output/Windows/Terralistic-server/")
    utils.copy(f"{project_path}Build/Terralistic-server.exe", f"{project_path}Output/Windows/Terralistic-server/Terralistic-server.exe")

    for file in utils.listDir(project_path + "Build/"):
        if file.endswith(".dll"):
            utils.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic/")
            utils.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic-server/")

    utils.copy("C:/Program Files/Git/usr/bin/patch.exe", f"{project_path}Output/Windows/Terralistic/patch.exe")
    utils.copy("C:/Program Files/Git/usr/bin/msys-2.0.dll", f"{project_path}Output/Windows/Terralistic/msys-2.0.dll")

    if utils.exists(f"{project_path}Output/Windows/Terralistic/Resources/"):
        utils.remove(f"{project_path}Output/Windows/Terralistic/Resources/")
    utils.move(f"{project_path}Build/Resources/", f"{project_path}Output/Windows/Terralistic/Resources/")
    utils.copy(f"{project_path}Resources/resourcePack/misc/Structures.asset", f"{project_path}Output/Windows/Terralistic-server/Structures.asset")
    utils.copy(f"{project_path}Resources/font.opa", f"{project_path}Output/Windows/Terralistic-server/font.opa")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        utils.system(f"\"{project_path}Output/Windows/Terralistic/Terralistic.exe\"")


def buildForLinux():
    utils.createDir(project_path + "Dependencies/")

    dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.zip", project_path + "Dependencies/glfw-3.3.7/", "glfw", f"cd {project_path}Dependencies/glfw-3.3.7/ && cmake -B build && cd build && make")

    utils.createDir(project_path + "Build/")
    utils.system(f"cd {project_path}Build/ && cmake .. && make -j$(nproc)")

    utils.createDir(project_path + "Output/")
    utils.createDir(project_path + "Output/Linux/")
    utils.createDir(project_path + "Output/Linux/Terralistic")
    if utils.exists(project_path + "Output/Linux/Terralistic/"):
        utils.remove(project_path + "Output/Linux/Terralistic/")
    utils.copy(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic/Resources/")
    utils.copy(project_path + "Build/Terralistic", project_path + "Output/Linux/Terralistic/Terralistic")

    utils.createDir(project_path + "Output/Linux/Terralistic-server")
    if utils.exists(project_path + "Output/Linux/Terralistic-server/"):
        utils.remove(project_path + "Output/Linux/Terralistic-server/")
    utils.copy(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic-server/Resources/")
    utils.copy(project_path + "Build/Terralistic-server", project_path + "Output/Linux/Terralistic-server/Terralistic-server")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        utils.system(project_path + "Output/Linux/Terralistic/Terralistic")


if sys.platform == "darwin":
    buildForMacOS()
elif sys.platform == "linux":
    buildForLinux()
elif sys.platform == "win32":
    buildForWindows()
else:
    print(f"{sys.platform} is not supported by this build script!")
