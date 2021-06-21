cmake_minimum_required(VERSION 3.10)
set (CMAKE_CXX_STANDARD 17)
project(Terralistic)
project(Terralistic-server)

if(NOT CMAKE_BUILD_TYPE STREQUAL Release)
    add_compile_definitions(DEVELOPER_MODE)
endif()

if(WIN32)
    if(CMAKE_BUILD_TYPE STREQUAL Release)
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Ofast")
    endif()
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread -I/usr/include/SDL2")
    if(CMAKE_BUILD_TYPE STREQUAL Release)
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Ofast")
    endif()
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")
endif()

file(GLOB_RECURSE TerralisticSrc CONFIGURE_DEPENDS Client/*.cpp Server/*.cpp Libraries/*.cpp)
list(REMOVE_ITEM TerralisticSrc ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)
file(GLOB_RECURSE TerralisticServerSrc CONFIGURE_DEPENDS Server/*.cpp Libraries/*.cpp)

add_executable(Terralistic WIN32 ${TerralisticSrc})
add_executable(Terralistic-server ${TerralisticServerSrc})

FILE(GLOB_RECURSE new_list *.hpp *.h)
SET(TerralisticInclude "")
FOREACH(file_path ${new_list})
    GET_FILENAME_COMPONENT(dir_path ${file_path} PATH)
    SET(TerralisticInclude ${TerralisticInclude} ${dir_path})
ENDFOREACH()
LIST(REMOVE_DUPLICATES TerralisticInclude)

include_directories(Terralistic ${TerralisticInclude})
include_directories(Terralistic-server ${TerralisticInclude})

if(WIN32)
    target_link_libraries(Terralistic
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2main.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2_image-2.0.5/lib/x64/SDL2_image.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2_ttf-2.0.15/lib/x64/SDL2_ttf.lib
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
file(COPY Client/Resources/Structures.asset DESTINATION .)
