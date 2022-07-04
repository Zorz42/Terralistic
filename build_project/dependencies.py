import urllib.request
import zipfile
from . import utils


def installDependency(url: str, directory: str, name: str, command: str = "", create_dir: bool = False):
    if not utils.exists(directory):
        print(f"Downloading dependency \"{name}\"")

        file = f"{name}.zip"

        with urllib.request.urlopen(url) as request:
            with open(file, 'wb') as download:
                download.write(request.read())

        if create_dir:
            utils.createDir(directory)

        dir_to_extract = directory
        if not create_dir:
            dir_to_extract = utils.getParentDir(directory)
        with zipfile.ZipFile(file, "r") as zip_file:
            zip_file.extractall(dir_to_extract)

        utils.remove(file)

        if command:
            utils.system(command)
    else:
        print(f"Dependency \"{name}\" is already installed.")
