import utils
import dependencies
import sys


def buildForWindows(project_path):
    utils.createDir(project_path + "Dependencies/")

    dependencies.installDependency("https://github.com/madler/zlib/archive/refs/heads/master.zip", project_path + "Dependencies/zlib-master/", "zlib", f"\"\"C:/Program Files/Microsoft Visual Studio/2022/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Dependencies/zlib-master/ && cmake -DCMAKE_INSTALL_PREFIX=. -G \"Visual Studio 17 2022\" -A Win32 . && cmake --build . --config Release --target install\"")
    dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.WIN32.zip", project_path + "Dependencies/glfw-3.3.7.bin.WIN32/", "glfw")
    dependencies.installDependency("https://netix.dl.sourceforge.net/project/gnuwin32/patch/2.5.9-7/patch-2.5.9-7-bin.zip", project_path + "Dependencies/patch-2.5.9-7-bin/", "patch", create_dir=True)

    utils.createDir(project_path + "Build/")

    utils.system(f"\"\"C:/Program Files/Microsoft Visual Studio/2022/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Build/ && cmake -DCMAKE_BUILD_TYPE=Release -G \"CodeBlocks - NMake Makefiles\" .. && cmake --build .\"")

    utils.createDir(project_path + "Output/Windows/Terralistic")
    utils.copy(f"{project_path}Build/Terralistic.exe", f"{project_path}Output/Windows/Terralistic/Terralistic.exe")

    utils.createDir(project_path + "Output/Windows/Terralistic-server/")
    utils.copy(f"{project_path}Build/Terralistic-server.exe", f"{project_path}Output/Windows/Terralistic-server/Terralistic-server.exe")

    for file in utils.listDir(project_path + "Build/"):
        if file.endswith(".dll"):
            utils.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic/")
            utils.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic-server/")

    utils.copy(project_path + "Dependencies/patch-2.5.9-7-bin/bin/patch.exe", f"{project_path}Output/Windows/Terralistic/patch.exe")

    if utils.exists(f"{project_path}Output/Windows/Terralistic/Resources/"):
        utils.remove(f"{project_path}Output/Windows/Terralistic/Resources/")
    utils.move(f"{project_path}Build/Resources/", f"{project_path}Output/Windows/Terralistic/Resources/")
    utils.copy(f"{project_path}Resources/resourcePack/misc/Structures.asset", f"{project_path}Output/Windows/Terralistic-server/Structures.asset")
    utils.copy(f"{project_path}Resources/font.opa", f"{project_path}Output/Windows/Terralistic-server/font.opa")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        utils.system(f"\"{project_path}Output/Windows/Terralistic/Terralistic.exe\"")