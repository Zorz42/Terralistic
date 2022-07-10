from . import macos
from . import windows
from . import linux
from . import utils
import sys
import os


def main():
    project_path = utils.getParentDir(utils.getParentDir(__file__))
    os.system("")  # Enables ansi escape codes

    arg = ""
    if len(sys.argv) > 1:
        arg = sys.argv[1]

    if sys.platform == "darwin":
        macos.buildForMacOS(project_path, arg)
    elif sys.platform == "linux":
        linux.buildForLinux(project_path, arg)
    elif sys.platform == "win32":
        windows.buildForWindows(project_path, arg)
    else:
        print(f"{sys.platform} is not supported by this build script!")
