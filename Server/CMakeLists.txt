cmake_minimum_required(VERSION 3.17)
project(TerralisticServer)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-O0 -static -Wall")
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

file(GLOB INCLUDES "Sources/*/Headers/")
file(GLOB INCLUDES2 "ClientLibraries/*/Headers/")
file(GLOB LIB_INCLUDES "Libraries/*/Headers/")

link_directories(OtherLibs)

file(GLOB SOURCES "Sources/*.cpp")
file(GLOB SOURCES2 "ClientLibraries/*/*.cpp")
file(GLOB SOURCES_LIBS "Libraries/*/*.cpp")

add_executable(TerralisticServer ${SOURCES} ${SOURCES2} ${SOURCES_LIBS})

include_directories(${INCLUDES} ${INCLUDES2} ${LIB_INCLUDES} OtherLibs OtherLibs/SDL2 ClientLibraries)

target_link_libraries(TerralisticServer AdvApi32.lib Mswsock.lib Ws2_32.lib)
