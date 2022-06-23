import urllib.request
import zipfile
import Scripts.utils as utils


def installDependency(url, directory, name, command=None):
    if not utils.exists(directory):
        print(f"Downloading {name}")

        file = f"{name}.zip"

        with urllib.request.urlopen(url) as request:
            with open(file, 'wb') as download:
                download.write(request.read())

        with zipfile.ZipFile(file, "r") as zip_file:
            zip_file.extractall(utils.getDir(directory))

        utils.remove(file)

        if command is not None:
            utils.system(command)
