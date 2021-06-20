cmake_minimum_required(VERSION 3.10)
project(Terralistic)
set (CMAKE_CXX_STANDARD 17)

add_compile_definitions(DEVELOPER_MODE)

if(WIN32)
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread -O2 -I/usr/include/SDL2")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")
endif()

file(GLOB_RECURSE TerralisticSrc CONFIGURE_DEPENDS Client/*.cpp Libraries/*.cpp Server/*.cpp)
list(REMOVE_ITEM TerralisticSrc ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)

add_executable(Terralistic WIN32 ${TerralisticSrc})

FILE(GLOB_RECURSE new_list *.hpp *.h)
SET(TerralisticInclude "")
FOREACH(file_path ${new_list})
    GET_FILENAME_COMPONENT(dir_path ${file_path} PATH)
    SET(TerralisticInclude ${TerralisticInclude} ${dir_path})
ENDFOREACH()
LIST(REMOVE_DUPLICATES TerralisticInclude)

include_directories(Terralistic ${TerralisticInclude})

if(WIN32)
    set(CMAKE_CXX_COMPILER "C:\\msys64\\mingw64\\bin\\clang++.exe")

    target_link_libraries(Terralistic
            ${PROJECT_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2.lib
            ${PROJECT_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2main.lib
            ${PROJECT_SOURCE_DIR}/Dependencies/Windows/SDL2_image-2.0.5/lib/x64/SDL2_image.lib
            ${PROJECT_SOURCE_DIR}/Dependencies/Windows/SDL2_ttf-2.0.15/lib/x64/SDL2_ttf.lib
            Ws2_32.lib
            )
    file(COPY
            Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/SDL2_image.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/libjpeg-9.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/libpng16-16.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/libtiff-5.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/libwebp-7.dll
            Dependencies/Windows/SDL2_image-2.0.5/lib/x64/zlib1.dll
            Dependencies/Windows/SDL2_ttf-2.0.15/lib/x64/libfreetype-6.dll
            Dependencies/Windows/SDL2_ttf-2.0.15/lib/x64/SDL2_ttf.dll
            DESTINATION .)
else()
    target_link_libraries(Terralistic SDL2 SDL2_ttf SDL2_image)
endif()

file(COPY Client/Resources DESTINATION .)
