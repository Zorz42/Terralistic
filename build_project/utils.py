import os
import shutil


def system(command: str):
    print(f"Running command \"{command}\"")
    result = os.system(command)
    if result != 0:
        raise Exception(f"Command: \"{command}\" exited with exit code {result}. Aborting.")


def createDir(path: str):
    if not os.path.isdir(getParentDir(path)):
        createDir(getParentDir(path))

    if not os.path.isdir(path):
        print(f"Creating directory \"{path}\"")
        os.mkdir(path)


def remove(path: str):
    if os.path.isdir(path):
        print(f"Removing directory \"{path}\"")
        shutil.rmtree(path)
    elif os.path.isfile(path):
        print(f"Removing file \"{path}\"")
        os.remove(path)


def copy(source: str, dest: str):
    if os.path.isdir(source):
        print(f"Copying directory from \"{source}\" to \"{dest}\"")
        shutil.copytree(source, dest)
    else:
        print(f"Copying file from \"{source}\" to \"{dest}\"")
        shutil.copy(source, dest)


def move(source: str, dest: str):
    if os.path.isdir(source):
        print(f"Moving directory from \"{source}\" to \"{dest}\"")
    else:
        print(f"Moving file from \"{source}\" to \"{dest}\"")
    shutil.move(source, dest)


def exists(path: str):
    return os.path.exists(path)


def getParentDir(path: str):
    return os.path.dirname(os.path.realpath(path)) + "/"


def listDir(path: str):
    return os.listdir(path)