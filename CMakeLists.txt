cmake_minimum_required(VERSION 3.17)
project(Terralistic)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-Ofast -static")
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

file(GLOB INCLUDES "Sources/*/Headers/")
file(GLOB INCLUDES2 "Sources/*/dependentModules/*/Headers/")
file(GLOB LIB_INCLUDES "Libraries/*/Headers/")

link_directories(OtherLibs)

file(GLOB SOURCES "Sources/*/*.cpp")
file(GLOB SOURCES2 "Sources/*/dependentModules/*/*.cpp")
file(GLOB SOURCES_LIBS "Libraries/*/*.cpp")

add_executable(Terralistic ${SOURCES} ${SOURCES2} ${SOURCES_LIBS})

include_directories(${INCLUDES}  ${INCLUDES2} ${LIB_INCLUDES} OtherLibs OtherLibs/SDL2)

target_link_libraries(Terralistic SDL2main SDL2 SDL2_image SDL2_ttf)
