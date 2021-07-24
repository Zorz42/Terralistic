set DEP_PATH=./Deps
set SFML_PATH=%DEP_PATH%/SFML

IF NOT EXIST %DEP_PATH% (
  mkdir %DEP_PATH%
)

IF NOT EXIST %SFML_PATH% (
  wget https://www.sfml-dev.org/files/SFML-2.5.1-windows-gcc-7.3.0-mingw-64-bit.zip
  compat /u "SFML-2.5.1-windows-gcc-7.3.0-mingw-64-bit.zip" /i /Q
  move SFML-2.5.1 %SFML_PATH%
)
