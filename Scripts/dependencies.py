import urllib.request
import zipfile
import Scripts.utils as utils


def installDependency(url, directory, name, command=None, create_dir=False):
    if not utils.exists(directory):
        print(f"Downloading {name}")

        file = f"{name}.zip"

        with urllib.request.urlopen(url) as request:
            with open(file, 'wb') as download:
                download.write(request.read())

        if create_dir:
            utils.createDir(directory)

        dir_to_extract = directory
        if not create_dir:
            dir_to_extract = utils.getDir(directory)
        with zipfile.ZipFile(file, "r") as zip_file:
            zip_file.extractall(dir_to_extract)

        utils.remove(file)

        if command is not None:
            utils.system(command)
