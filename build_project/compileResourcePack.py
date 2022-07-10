#!/usr/bin/env python3
import os
import sys
from . import utils
from . import createTexture
from . import pngToOpa

TEMPLATE_PREFIX = "Template_"


def compileResourcePack(input_resource_pack, output_resource_pack):
    if not input_resource_pack.endswith("/"):
        input_resource_pack += "/"
    if not output_resource_pack.endswith("/"):
        output_resource_pack += "/"

    utils.createDir(output_resource_pack)
    for file in os.listdir(input_resource_pack):
        input_file = input_resource_pack + file

        if file == ".DS_Store":
            continue

        if file.endswith(".png"):
            if file.startswith(TEMPLATE_PREFIX):
                output_file = output_resource_pack + file[len(TEMPLATE_PREFIX):-4] + ".opa"
            else:
                output_file = output_resource_pack + file[0:-4] + ".opa"
        else:
            output_file = output_resource_pack + file

        if os.path.exists(output_file) and os.path.getmtime(input_file) <= os.path.getmtime(output_file):
            continue

        if os.path.isdir(input_file):
            compileResourcePack(input_file, output_file)
        else:
            if file.endswith(".png"):
                if file.startswith(TEMPLATE_PREFIX):
                    temp_file = "._temp.png"
                    createTexture.generateBlockTexture(input_file, temp_file)
                    pngToOpa.pngToOpa(temp_file, output_file)
                    utils.remove(temp_file)
                else:
                    pngToOpa.pngToOpa(input_file, output_file)
            else:
                utils.copy(input_file, output_file)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: compileResourcePack.py [input] [output]")
    else:
        compileResourcePack(sys.argv[1], sys.argv[2])
