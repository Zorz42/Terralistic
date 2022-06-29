from Scripts import utils
from Scripts import dependencies
import sys


def buildForLinux(project_path):
    utils.createDir(project_path + "Dependencies/")

    dependencies.installDependency("https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.zip", project_path + "Dependencies/glfw-3.3.7/", "glfw", f"cd {project_path}Dependencies/glfw-3.3.7/ && cmake -B build && cd build && make")

    utils.createDir(project_path + "Build/")
    utils.system(f"cd {project_path}Build/ && cmake .. && make -j$(nproc)")

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