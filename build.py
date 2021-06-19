import os
import sys
import shutil

project_path = os.path.dirname(os.path.realpath(__file__)) + "/"


def createDir(path):
    os.makedirs(project_path + path, exist_ok=True)

if sys.platform == "darwin":
    createDir(project_path + "Dependencies/")
    if not os.path.exists(project_path + "Dependencies/MacOS/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-MacOS-Dependencies {project_path}Dependencies/MacOS/")
    else:
        os.system(f"git -C {project_path}Dependencies/MacOS/ pull --rebase")

    os.system(f"xcodebuild build -project {project_path}Terralistic.xcodeproj ARCHS=x86_64 ONLY_ACTIVE_ARCH=NO -scheme Terralistic BUILD_DIR={project_path}Temp")

    createDir("Output/MacOS/")

    shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/", ignore_errors=True)
    shutil.move(project_path + "Temp/Debug/Terralistic.app/", project_path + "Output/MacOS/")
    shutil.rmtree(project_path + "Temp/")
elif sys.platform == "linux":
    os.system("sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev")

    createDir("Build/")
    os.system(f"cd {project_path}Build/ && cmake -DCMAKE_CXX_COMPILER=/usr/bin/clang++ .. && make -j$(nproc)")

    createDir("Output/Linux/Terralistic")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Terralistic"):
        os.remove(project_path + "Output/Linux/Terralistic/Terralistic")
    shutil.move(project_path + "Build/Terralistic", project_path + "Output/Linux/Terralistic/")
    shutil.rmtree(project_path + "Output/Linux/Terralistic/Resources/", ignore_errors=True)
    shutil.move(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic/")

    shutil.copy("/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0", project_path + "Output/Linux/Terralistic/")
    shutil.copy("/usr/lib/x86_64-linux-gnu/libSDL2_image-2.0.so.0", project_path + "Output/Linux/Terralistic/")
    shutil.copy("/usr/lib/x86_64-linux-gnu/libSDL2_image-2.0.so.0", project_path + "Output/Linux/Terralistic/")
elif sys.platform == "win32":
    createDir(project_path + "Dependencies/")
    if not os.path.exists(project_path + "Dependencies/Windows/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-Windows-Dependencies {project_path}Dependencies/Windows/")
    else:
        os.system(f"git -C {project_path}Dependencies/Windows/ pull --rebase")

    createDir("Build/")
    clion_folder = ""
    for folder in os.listdir("C:\\Program Files\\JetBrains\\"):
        if folder.startswith("CLion"):
            clion_folder = folder
    cmake_path = f"\"C:\\Program Files\\JetBrains\\{clion_folder}\\bin\\cmake\\win\\bin\\cmake.exe\""
    print("Building with ", cmake_path)

    os.system(f"cd {project_path}Build/ && {cmake_path} .. && {cmake_path} --build .")

    createDir("Output/Windows/Terralistic")

    if os.path.exists(project_path + "Output/Windows/Terralistic/"):
        os.remove(project_path + "Output/Windows/Terralistic/")
    shutil.move(project_path + "Build/Debug/Terralistic.exe", project_path + "Output/Windows/Terralistic/")
    for file in os.listdir(project_path + "Build/"):
        if file.endswith(".dll"):
            shutil.move(project_path + "Build/" + file, project_path + "Output/Windows/Terralistic/")

    shutil.rmtree(project_path + "Output/Windows/Terralistic/Resources/", ignore_errors=True)
    shutil.move(project_path + "Build/Resources/", project_path + "Output/Windows/Terralistic/Resources/")
else:
    print("Your current platform is not yet supported by this build script!")