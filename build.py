#!/usr/bin/env python3

import os
import sys
import shutil
import urllib.request
import tarfile
import zipfile

project_path = os.path.dirname(os.path.realpath(__file__)) + "/"


def createDir(path):
    os.makedirs(project_path + path, exist_ok=True)


if sys.platform == "darwin":
    createDir(project_path + "Dependencies/")
        
    if not os.path.exists(project_path + "Dependencies/glfw-3.3.7.bin.MACOS/"):
        print("Downloading glfw library")

        glfw_url = "https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.MACOS.zip"
        glfw_file = project_path + "glfw.tar.gz"

        with urllib.request.urlopen(glfw_url) as glfw_request:
            with open(glfw_file, 'wb') as glfw_download:
                glfw_download.write(glfw_request.read())

        with zipfile.ZipFile(glfw_file, "r") as glfw_zip:
            glfw_zip.extractall(f"{project_path}Dependencies/")

        os.remove(glfw_file)
    
    os.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic archive -jobs $(sysctl -n hw.ncpu) -archivePath {project_path}Terralistic.xcarchive")
    os.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic-server archive -configuration release -jobs $(sysctl -n hw.ncpu) -archivePath {project_path}Terralistic-server.xcarchive")

    createDir("Output/MacOS/")

    shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/", ignore_errors=True)
    os.system(f"xcodebuild -exportArchive -quiet -archivePath {project_path}Terralistic.xcarchive -exportPath {project_path}Terralistic.app/ -exportOptionsPlist {project_path}exportOptions.plist")
    shutil.copytree(f"{project_path}Terralistic.app/Terralistic.app/", f"{project_path}Output/MacOS/Terralistic.app/")

    shutil.rmtree(project_path + "Output/MacOS/Terralistic-server.app/", ignore_errors=True)
    os.system(f"xcodebuild -exportArchive -quiet -archivePath {project_path}Terralistic-server.xcarchive -exportPath {project_path}Terralistic-server.app/ -exportOptionsPlist {project_path}exportOptions.plist")
    shutil.copytree(f"{project_path}Terralistic-server.app/Terralistic-server.app/", f"{project_path}Output/MacOS/Terralistic-server.app/")

    shutil.rmtree(project_path + "Terralistic.xcarchive/")
    shutil.rmtree(project_path + "Terralistic-server.xcarchive/")

    shutil.rmtree(project_path + "Terralistic.app/")
    shutil.rmtree(project_path + "Terralistic-server.app/")

elif sys.platform == "linux":
    createDir("Dependencies/")

    if not os.path.exists(project_path + "Dependencies/glfw-3.3.7.bin.MACOS/"):
        print("Downloading glfw library")

        glfw_url = "https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.zip"
        glfw_file = project_path + "glfw.tar.gz"

        with urllib.request.urlopen(glfw_url) as glfw_request:
            with open(glfw_file, 'wb') as glfw_download:
                glfw_download.write(glfw_request.read())

        with zipfile.ZipFile(glfw_file, "r") as glfw_zip:
            glfw_zip.extractall(f"{project_path}Dependencies/")

        os.remove(glfw_file)

        os.system(f"cd {project_path}Dependencies/glfw-3.3.7/ && cmake -B build && cd build && make")

    createDir("Build/")
    os.system(f"cd {project_path}Build/ && cmake .. && make -j$(nproc)")

    createDir("Output/Linux/Terralistic")
    if os.path.exists(project_path + "Output/Linux/Terralistic/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic/")
    shutil.copytree(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic/Resources/")
    shutil.copy(project_path + "Build/Terralistic", project_path + "Output/Linux/Terralistic/Terralistic")

    createDir("Output/Linux/Terralistic-server")
    if os.path.exists(project_path + "Output/Linux/Terralistic-server/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic-server/")
    shutil.copytree(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic-server/Resources/")
    shutil.copy(project_path + "Build/Terralistic-server", project_path + "Output/Linux/Terralistic-server/Terralistic-server")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        os.system(project_path + "Output/Linux/Terralistic/Terralistic")

elif sys.platform == "win32":
    createDir("Dependencies/")

    if not os.path.exists(project_path + "Dependencies/zlib-master/"):
        print("Downloading zlib library")

        zlib_url = "https://github.com/madler/zlib/archive/refs/heads/master.zip"
        zlib_file = project_path + "zlib.zip"

        with urllib.request.urlopen(zlib_url) as zlib_request:
            with open(zlib_file, 'wb') as zlib_download:
                zlib_download.write(zlib_request.read())

        with zipfile.ZipFile(zlib_file, "r") as zlib_zip:
            zlib_zip.extractall(f"{project_path}Dependencies/")

        os.remove(zlib_file)

        os.system(f"\"\"C:/Program Files (x86)/Microsoft Visual Studio/2019/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Dependencies/zlib-master/ && cmake -DCMAKE_INSTALL_PREFIX=. -G \"Visual Studio 16 2019\" . && cmake --build . --config Release --target install /m\"")
        
        #with open(project_path + "Dependencies/zlib/include/zconf.h", "r") as header:
        #    lines = header.readlines()
        #lines[286] = "#if 0\n"
        #with open(project_path + "Dependencies/zlib/include/zconf.h", "w") as header:
        #    header.writelines(lines)

    if not os.path.exists(project_path + "Dependencies/glfw-3.3.7.bin.WIN32/"):
        print("Downloading glfw library")

        glfw_url = "https://github.com/glfw/glfw/releases/download/3.3.7/glfw-3.3.7.bin.WIN32.zip"
        glfw_file = project_path + "glfw.tar.gz"

        with urllib.request.urlopen(glfw_url) as glfw_request:
            with open(glfw_file, 'wb') as glfw_download:
                glfw_download.write(glfw_request.read())

        with zipfile.ZipFile(glfw_file, "r") as glfw_zip:
            glfw_zip.extractall(f"{project_path}Dependencies/")

        os.remove(glfw_file)

    createDir("Build/")

    os.system(f"\"\"C:/Program Files (x86)/Microsoft Visual Studio/2019/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Build/ && cmake -DCMAKE_BUILD_TYPE=Release -G \"CodeBlocks - NMake Makefiles\" .. && cmake --build . /m\"")

    createDir("Output/Windows/Terralistic/")
    shutil.copy(f"{project_path}Build/Terralistic.exe", f"{project_path}Output/Windows/Terralistic/Terralistic.exe")

    createDir("Output/Windows/Terralistic-server/")
    shutil.copy(f"{project_path}Build/Terralistic-server.exe", f"{project_path}Output/Windows/Terralistic-server/Terralistic-server.exe")

    for file in os.listdir(project_path + "Build/"):
        if file.endswith(".dll"):
            shutil.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic/")
            shutil.copy(f"{project_path}Build/{file}", f"{project_path}Output/Windows/Terralistic-server/")

    shutil.copy("C:/Program Files/Git/usr/bin/patch.exe", f"{project_path}Output/Windows/Terralistic/patch.exe")
    shutil.copy("C:/Program Files/Git/usr/bin/msys-2.0.dll", f"{project_path}Output/Windows/Terralistic/msys-2.0.dll")

    if os.path.exists(f"{project_path}Output/Windows/Terralistic/Resources/"):
        shutil.rmtree(f"{project_path}Output/Windows/Terralistic/Resources/")
    shutil.move(f"{project_path}Build/Resources/", f"{project_path}Output/Windows/Terralistic/Resources/")
    shutil.copy(f"{project_path}Resources/resourcePack/misc/Structures.asset", f"{project_path}Output/Windows/Terralistic-server/Structures.asset")
    shutil.copy(f"{project_path}Resources/font.opa", f"{project_path}Output/Windows/Terralistic-server/font.opa")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        os.system(f"\"{project_path}Output/Windows/Terralistic/Terralistic.exe\"")
else:
    print("Your current platform is not yet supported by this build script!")
