import os
import sys
import shutil

if sys.platform == "darwin":
    project_path = os.path.dirname(os.path.realpath(__file__)) + "/"
    if not os.path.exists(project_path + "Dependencies/"):
        os.mkdir(project_path + "Dependencies/")
    if not os.path.exists(project_path + "Dependencies/MacOS/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-MacOS-Dependecies {project_path + 'Dependencies/MacOS/'}")
    else:
        os.system(f"git -C {project_path + 'Dependencies/MacOS/'} pull --rebase")

    os.system(f"xcodebuild build -project {project_path + 'Terralistic.xcodeproj'} ARCHS=x86_64 ONLY_ACTIVE_ARCH=NO -scheme Terralistic BUILD_DIR={project_path + 'Temp'}")

    if not os.path.exists(project_path + "Output/"):
        os.mkdir(project_path + "Output")

    if not os.path.exists(project_path + "Output/MacOS/"):
        os.mkdir(project_path + "Output/MacOS/")

    if os.path.exists(project_path + "Output/MacOS/Terralistic.app/"):
        shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/")
    os.system(f"mv {project_path + 'Temp/Debug/Terralistic.app/'} {project_path + 'Output/MacOS/'}")

    shutil.rmtree(project_path + "Temp/")
elif sys.platform == "linux":
    os.system("sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev")

    project_path = os.path.dirname(os.path.realpath(__file__)) + "/"
    if not os.path.exists(project_path + "Build/"):
        os.mkdir(project_path + "Build/")

    os.system(f"cd {project_path + 'Build/'} && cmake .. && make -j$(nproc)")

    if not os.path.exists(project_path + "Output/"):
        os.mkdir(project_path + "Output")

    if not os.path.exists(project_path + "Output/Linux/"):
        os.mkdir(project_path + "Output/Linux/")

    if not os.path.exists(project_path + "Output/Linux/Terralistic/"):
        os.mkdir(project_path + "Output/Linux/Terralistic/")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Resources/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic/Resources/")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Terralistic"):
        os.remove(project_path + "Output/Linux/Terralistic/Terralistic")

    os.system(f"mv {project_path + 'Build/Terralistic'} {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"mv {project_path + 'Build/Resources/'} {project_path + 'Output/Linux/Terralistic/'}")
else:
    print("Your current platform is not yet supported by this build script!")
