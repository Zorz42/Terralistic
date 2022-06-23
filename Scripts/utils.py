import os


def system(command: str):
    print(f"Running command \"{command}\"")
    result = os.system(command)
    if result != 0:
        raise Exception(f"Command: \"{command}\" exited with exit code {result}. Aborting.")


def createDir(path):
    if not os.path.isdir(path):
        print(f"Creating directory \"{path}\"")
        os.mkdir(path)
