import os
import sys
import shutil

project_path = os.path.dirname(os.path.realpath(__file__)) + "/"


def createDir(path):
    os.makedirs(project_path + path, exist_ok=True)


if sys.platform == "darwin":
    createDir("Dependencies/")
    if not os.path.exists(project_path + "Dependencies/MacOS/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-MacOS-Dependencies {project_path + 'Dependencies/MacOS/'}")
    else:
        os.system(f"git -C {project_path + 'Dependencies/MacOS/'} pull --rebase")

    os.system(f"xcodebuild build -project {project_path + 'Terralistic.xcodeproj'} ARCHS=x86_64 ONLY_ACTIVE_ARCH=NO -scheme Terralistic BUILD_DIR={project_path + 'Temp'}")

    createDir("Output/MacOS/")

    if os.path.exists(project_path + "Output/MacOS/Terralistic.app/"):
        shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/")
    os.system(f"mv {project_path + 'Temp/Debug/Terralistic.app/'} {project_path + 'Output/MacOS/'}")

    shutil.rmtree(project_path + "Temp/")
elif sys.platform == "linux":
    os.system("sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev")

    createDir("Build/")
    os.system(f"cd {project_path + 'Build/'} && cmake .. && make -j$(nproc)")

    createDir("Output/Linux/Terralistic")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Resources/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic/Resources/")

    if os.path.exists(project_path + "Output/Linux/Terralistic/Terralistic"):
        os.remove(project_path + "Output/Linux/Terralistic/Terralistic")

    os.system(f"mv {project_path + 'Build/Terralistic'} {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"mv {project_path + 'Build/Resources/'} {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"cp /usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0 {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"cp /usr/lib/x86_64-linux-gnu/libSDL2_image-2.0.so.0 {project_path + 'Output/Linux/Terralistic/'}")
    os.system(f"cp /usr/lib/x86_64-linux-gnu/libSDL2_ttf-2.0.so.0 {project_path + 'Output/Linux/Terralistic/'}")
elif sys.platform == "win32":
    createDir("Dependencies/")
    if not os.path.exists(project_path + "Dependencies/Windows/"):
        os.system(f"git clone https://github.com/Zorz42/Terralistic-Windows-Dependencies {project_path + 'Dependencies/Windows/'}")
    else:
        os.system(f"git -C {project_path + 'Dependencies/Windows/'} pull --rebase")

    createDir("Build/")
    clion_folder = ""
    for folder in os.listdir("C:\\Program Files\\JetBrains\\"):
        if folder.startswith("CLion"):
            clion_folder = folder
    cmake_path = f"\"C:\\Program Files\\JetBrains\\{clion_folder}\\bin\\cmake\\win\\bin\\cmake.exe\""
    print("Building with ", cmake_path)

    os.system(f"cd {project_path + 'Build/'} && {cmake_path} .. && {cmake_path} --build .")

    createDir("Output/Windows/Terralistic")

    shutil.copy("Build/Debug/Terralistic.exe", "Output/Windows/Terralistic/")
    for file in os.listdir("Build/"):
        if file.endswith(".dll"):
            shutil.copy("Build/" + file, "Output/Windows/Terralistic/")

    shutil.copytree("Build/Resources/", "Output/Windows/Terralistic/Resources/", dirs_exist_ok=True)
else:
    print("Your current platform is not yet supported by this build script!")
