#!/usr/bin/env python3

from Scripts.macos import buildForMacOS
from Scripts.windows import buildForWindows
from Scripts.linux import buildForLinux
import Scripts.utils as utils
import sys
import os

project_path = utils.getParentDir(__file__)
os.system("")  # Enables ansi escape codes

if sys.platform == "darwin":
    buildForMacOS(project_path)
elif sys.platform == "linux":
    buildForLinux(project_path)
elif sys.platform == "win32":
    buildForWindows(project_path)
else:
    print(f"{sys.platform} is not supported by this build script!")
