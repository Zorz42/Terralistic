import os
import sys
import shutil

project_path = os.path.dirname(os.path.realpath(__file__)) + "/"

def createDir(path):
    if not os.path.exists(project_path + path):
        os.mkdir(project_path + path)


if sys.platform == "darwin":
    createDir("Dependencies/")
    if not os.path.exists(project_path + "Dependencies/MacOS/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-MacOS-Dependencies {project_path + 'Dependencies/MacOS/'}")
    else:
        os.system(f"git -C {project_path + 'Dependencies/MacOS/'} pull --rebase")

    os.system(
        f"xcodebuild build -project {project_path + 'Terralistic.xcodeproj'} ARCHS=x86_64 ONLY_ACTIVE_ARCH=NO -scheme Terralistic BUILD_DIR={project_path + 'Temp'}")

    createDir("Output/")
    createDir("Output/MacOS/")

    if os.path.exists(project_path + "Output/MacOS/Terralistic.app/"):
        shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/")
    os.system(f"mv {project_path + 'Temp/Debug/Terralistic.app/'} {project_path + 'Output/MacOS/'}")

    shutil.rmtree(project_path + "Temp/")
elif sys.platform == "linux":
    os.system("sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev")

    createDir("Dependencies/")
    if not os.path.exists(project_path + "Dependencies/Linux/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-Linux-Dependencies {project_path + 'Dependencies/Linux/'}")
    else:
        os.system(f"git -C {project_path + 'Dependencies/Linux/'} pull --rebase")

    createDir("Build/")

    os.system(f"cd {project_path + 'Build/'} && cmake .. && make -j$(nproc)")

    createDir("Output/")
    createDir("Output/Linux/")
    createDir("Output/Linux/Terralistic")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Resources/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic/Resources/")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Terralistic"):
        os.remove(project_path + "Output/Linux/Terralistic/Terralistic")

    os.system(f"mv {project_path + 'Build/Terralistic'} {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"mv {project_path + 'Build/Resources/'} {project_path + 'Output/Linux/Terralistic/'}")
else:
    print("Your current platform is not yet supported by this build script!")
