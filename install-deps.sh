#!/usr/bin/bash

DEP_PATH=./Deps
SFML_PATH=$DEP_PATH/SFML

if test -d "$DEP_PATH"; then
    echo
else
    mkdir "$DEP_PATH"
fi

curl_deps() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl https://www.sfml-dev.org/files/SFML-2.5.1-linux-gcc-64-bit.tar.gz --output sfml.tar.gz
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    curl -https://www.sfml-dev.org/files/SFML-2.5.1-macOS-clang.tar.gz --output sfml.tar.gz
  else
    echo Os Not Supported!
  fi
}

if test -d "$SFML_PATH"; then
    echo Deps already installed!
else
  curl_deps
  tar -xvf sfml.tar.gz
  mv SFML* $SFML_PATH
  rm sfml.tar.gz
fi
