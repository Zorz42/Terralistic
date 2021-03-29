cmake_minimum_required(VERSION 3.17)
project(Terralistic)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-O2 -static -Wall")
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

file(GLOB INCLUDES "Sources/*/")
file(GLOB INCLUDES2 "Sources/*/*/")
file(GLOB INCLUDES3 "Sources/*/*/*/")
file(GLOB LIB_INCLUDES "*/")
file(GLOB CORE_INCLUDES "../Core/*/")
file(GLOB CORE_INCLUDES2 "../Core/*/*/")

link_directories(OtherLibs)

file(GLOB SOURCES "Sources/*/*.cpp")
file(GLOB SOURCES2 "Sources/*/*/*.cpp")
file(GLOB SOURCES3 "Sources/*/*/*/*.cpp")
file(GLOB SOURCES_LIBS "Graphics/*.cpp")
file(GLOB SOURCES_CORE "../Core/*/*.cpp")
file(GLOB SOURCES_CORE2 "../Core/*/*/*.cpp")

add_executable(Terralistic ${SOURCES} ${SOURCES2} ${SOURCES3} ${SOURCES_LIBS} ${SOURCES_CORE} ${SOURCES_CORE2} Sources/main/main.cpp)

include_directories(${INCLUDES} ${INCLUDES2} ${INCLUDES3} ${LIB_INCLUDES} ${CORE_INCLUDES} ${CORE_INCLUDES2} ../Core/ OtherLibs OtherLibs/SDL2)

target_link_libraries(Terralistic SDL2main SDL2 SDL2_image SDL2_ttf AdvApi32.lib Mswsock.lib Ws2_32.lib)
