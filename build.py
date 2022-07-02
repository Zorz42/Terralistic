#!/usr/bin/env python3

from Scripts.macos import buildForMacOS
from Scripts.windows import buildForWindows
from Scripts.linux import buildForLinux
import Scripts.utils as utils
import sys
import os


def main():
    project_path = utils.getParentDir(__file__)
    os.system("")  # Enables ansi escape codes

    arg = ""
    if len(sys.argv) > 1:
        arg = sys.argv[1]

    if sys.platform == "darwin":
        buildForMacOS(project_path, arg)
    elif sys.platform == "linux":
        buildForLinux(project_path, arg)
    elif sys.platform == "win32":
        buildForWindows(project_path, arg)
    else:
        print(f"{sys.platform} is not supported by this build script!")


if __name__ == "__main__":
    main()
