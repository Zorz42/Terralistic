cmake_minimum_required(VERSION 3.10)

project(Terralistic)

set (CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread")

file(GLOB_RECURSE TerralisticSrc CONFIGURE_DEPENDS Client/*.cpp Libraries/*.cpp Server/*.cpp)
list(REMOVE_ITEM TerralisticSrc ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)

add_executable(Terralistic ${TerralisticSrc})

FILE(GLOB_RECURSE new_list *.hpp *.h)
SET(TerralisticInclude "")
FOREACH(file_path ${new_list})
    GET_FILENAME_COMPONENT(dir_path ${file_path} PATH)
    SET(TerralisticInclude ${TerralisticInclude} ${dir_path})
ENDFOREACH()
LIST(REMOVE_DUPLICATES TerralisticInclude)

include_directories(Terralistic ${TerralisticInclude})

target_link_libraries(Terralistic SDL2 SDL2_ttf SDL2_image)
