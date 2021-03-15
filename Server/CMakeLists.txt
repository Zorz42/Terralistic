cmake_minimum_required(VERSION 3.17)
project(TerralisticServer)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-O2 -static -Wall")
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

file(GLOB INCLUDES "*/")
file(GLOB INCLUDES_CORE "../Core/*/")
file(GLOB INCLUDES_CORE2 "../Core/*/*/")

link_directories(OtherLibs)

file(GLOB SOURCES "*/*.cpp")
file(GLOB SOURCES_CORE "../Core/*/*.cpp")
file(GLOB SOURCES_CORE2 "../Core/*/*/*.cpp")

add_executable(TerralisticServer ${SOURCES} ${SOURCES_CORE} ${SOURCES_CORE2})

include_directories(${INCLUDES} ${INCLUDES_CORE} ${INCLUDES_CORE2} ../Core/)

target_link_libraries(TerralisticServer AdvApi32.lib Mswsock.lib Ws2_32.lib)
