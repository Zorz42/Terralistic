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

    if not os.path.exists(project_path + "Dependencies/SFML-2.5.1-macos-clang/"):
        print("Downloading SFML libraries")

        sfml_url = "https://www.sfml-dev.org/files/SFML-2.5.1-macOS-clang.tar.gz"
        sfml_file = project_path + "sfml.tar.gz"

        with urllib.request.urlopen(sfml_url) as sfml_request:
            with open(sfml_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with tarfile.open(sfml_file, "r:gz") as sfml_tar:
            sfml_tar.extractall(project_path + "Dependencies/")

        os.remove(sfml_file)
    
    os.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic -configuration release -jobs $(sysctl -n hw.ncpu) BUILD_DIR={project_path}")
    os.system(f"xcodebuild build -quiet -project {project_path}Terralistic.xcodeproj -scheme Terralistic-server -configuration release -jobs $(sysctl -n hw.ncpu) BUILD_DIR={project_path}")

    createDir("Output/MacOS/")

    shutil.rmtree(project_path + "Output/MacOS/Terralistic.app/", ignore_errors=True)
    shutil.move(project_path + "Release/Terralistic.app/", project_path + "Output/MacOS/")

    shutil.rmtree(project_path + "Output/MacOS/Terralistic-server.app/", ignore_errors=True)
    shutil.move(project_path + "Release/Terralistic-server.app/", project_path + "Output/MacOS/")

    shutil.rmtree(project_path + "Release/")


elif sys.platform == "linux":
    createDir("Dependencies/")

    if not os.path.exists(project_path + "Dependencies/SFML-2.5.1/"):
        print("Downloading SFML libraries")

        sfml_url = "https://www.sfml-dev.org/files/SFML-2.5.1-linux-gcc-64-bit.tar.gz"
        sfml_file = project_path + "sfml.tar.gz"

        with urllib.request.urlopen(sfml_url) as sfml_request:
            with open(sfml_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with tarfile.open(sfml_file, "r") as sfml_tar:
            sfml_tar.extractall(project_path + "Dependencies/")

        os.remove(sfml_file)

    createDir("Build/")
    os.system(f"cd {project_path}Build/ && cmake .. && make -j$(nproc)")

    createDir("Output/Linux/Terralistic")
    if os.path.exists(project_path + "Output/Linux/Terralistic/"):
        shutil.rmtree(project_path + "Output/Linux/Terralistic/")
    shutil.copytree(project_path + "Build/Resources/", project_path + "Output/Linux/Terralistic/Resources/")
    shutil.copy(project_path + "Build/Terralistic", project_path + "Output/Linux/Terralistic/Terralistic")

    createDir("Output/Linux/Terralistic-server")
    shutil.copy(project_path + "Build/Terralistic-server", project_path + "Output/Linux/Terralistic-server/Terralistic-server")
    shutil.copy(project_path + "Build/Structures.asset", project_path + "Output/Linux/Terralistic-server/Structures.asset")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        os.system(project_path + "Output/Linux/Terralistic/Terralistic")

elif sys.platform == "win32":
    createDir("Dependencies/")

    if not os.path.exists(project_path + "Dependencies/SFML-2.5.1/"):
        print("Downloading SFML libraries")

        sfml_url = "https://www.sfml-dev.org/files/SFML-2.5.1-windows-vc15-32-bit.zip"
        sfml_file = project_path + "sfml.zip"

        with urllib.request.urlopen(sfml_url) as sfml_request:
            with open(sfml_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with zipfile.ZipFile(sfml_file, "r") as sfml_zip:
            sfml_zip.extractall(project_path + "Dependencies/")

        os.remove(sfml_file)

    if not os.path.exists(project_path + "Dependencies/zlib/"):
        print("Downloading zlib libraries")

        os.mkdir(project_path + "Dependencies/zlib/")

        zlib_url = "https://deac-riga.dl.sourceforge.net/project/gnuwin32/zlib/1.2.3/zlib-1.2.3-bin.zip"
        zlib_file = project_path + "zlib.zip"

        with urllib.request.urlopen(zlib_url) as sfml_request:
            with open(zlib_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with zipfile.ZipFile(zlib_file, "r") as sfml_zip:
            sfml_zip.extractall(project_path + "Dependencies/zlib/")

        os.remove(zlib_file)

        zlib_url = "https://deac-fra.dl.sourceforge.net/project/gnuwin32/zlib/1.2.3/zlib-1.2.3-lib.zip"
        zlib_file = project_path + "zlib.zip"

        with urllib.request.urlopen(zlib_url) as sfml_request:
            with open(zlib_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with zipfile.ZipFile(zlib_file, "r") as sfml_zip:
            sfml_zip.extractall(project_path + "Dependencies/zlib/")

        os.remove(zlib_file)
        
        zlib_url = "https://netix.dl.sourceforge.net/project/gnuwin32/zlib/1.2.3/zlib-1.2.3-lib.zip"
        zlib_file = project_path + "zlib.zip"

        with urllib.request.urlopen(zlib_url) as sfml_request:
            with open(zlib_file, 'wb') as sfml_download:
                sfml_download.write(sfml_request.read())

        with zipfile.ZipFile(zlib_file, "r") as sfml_zip:
            sfml_zip.extractall(project_path + "Dependencies/zlib/")

        os.remove(zlib_file)

        lines = []
        with open(project_path + "Dependencies/zlib/include/zconf.h", "r") as header:
            lines = header.readlines()
        lines[286] = "#if 0\n"
        with open(project_path + "Dependencies/zlib/include/zconf.h", "w") as header:
            header.writelines(lines)

    createDir("Build/")

    os.system(f"\"\"C:/Program Files (x86)/Microsoft Visual Studio/2019/Community/Common7/Tools/VsDevCmd.bat\" && cd {project_path}Build/ && cmake -DCMAKE_BUILD_TYPE=Release -G \"CodeBlocks - NMake Makefiles\" .. && cmake --build .\"")

    createDir("Output/Windows/Terralistic/")
    shutil.copy(project_path + "Build/Terralistic.exe", project_path + "Output/Windows/Terralistic/Terralistic.exe")

    createDir("Output/Windows/Terralistic-server/")
    shutil.copy(project_path + "Build/Terralistic-server.exe", project_path + "Output/Windows/Terralistic-server/Terralistic-server.exe")

    for file in os.listdir(project_path + "Build/"):
        if file.endswith(".dll"):
            shutil.copy(project_path + "Build/" + file, project_path + "Output/Windows/Terralistic/")
            shutil.copy(project_path + "Build/" + file, project_path + "Output/Windows/Terralistic-server/")

    shutil.move(project_path + "Build/Resources/", project_path + "Output/Windows/Terralistic/Resources/")
    shutil.copy(project_path + "Build/Structures.asset", project_path + "Output/Windows/Terralistic-server/Structures.asset")

    if len(sys.argv) != 1 and sys.argv[1] == "run":
        os.system(project_path + "Output/Windows/Terralistic/Terralistic.exe")
else:
    print("Your current platform is not yet supported by this build script!")
