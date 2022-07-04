cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 20)
project(Terralistic-server CXX)
project(Terralistic CXX)

execute_process(COMMAND py -m build_project nobuild
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_custom_command(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/Build/Resources/"
        COMMAND "py -m build_project nobuild")

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(IS_DEBUG TRUE)
    set(IS_RELEASE FALSE)
else()
    set(IS_DEBUG FALSE)
    set(IS_RELEASE TRUE)
endif()

if(UNIX)
    set(CMAKE_LINKER_FLAGS "${CMAKE_LINKER_FLAGS} -lpthread")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread")
endif()

if(IS_DEBUG)
    add_compile_definitions(DEVELOPER_MODE NDEBUG)
else()
    if(WIN32)
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /O2")
    else()
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Ofast")
    endif()    
endif()

file(GLOB_RECURSE SRC_LIBRARIES CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Libraries/*.cpp "${CMAKE_CURRENT_SOURCE_DIR}/External Libraries/*.cpp" ${CMAKE_CURRENT_SOURCE_DIR}/Shared/*.cpp)
file(GLOB_RECURSE SRC_CLIENT CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Client/*.cpp)
file(GLOB_RECURSE SRC_SERVER CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Server/*.cpp)
list(REMOVE_ITEM SRC_SERVER ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)

SET(GLAD_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glad/)
SET(GLAD_INCLUDE_DIR ${GLAD_DIR}/include/)
SET(GLAD_SOURCES ${GLAD_DIR}/src/glad.c)

SET(PF_DIR ${CMAKE_SOURCE_DIR}/Dependencies/PlatformFolders-4.2.0/)
SET(PF_INCLUDE_DIR ${PF_DIR}/sago/)
SET(PF_SOURCES ${PF_DIR}/sago/platform_folders.cpp)

add_executable(Terralistic ${SRC_LIBRARIES} ${SRC_CLIENT} ${SRC_SERVER} ${PF_SOURCES} ${GLAD_SOURCES})
add_executable(Terralistic-server ${SRC_LIBRARIES} ${SRC_SERVER} ${PF_SOURCES} ${GLAD_SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp ${CMAKE_CURRENT_SOURCE_DIR})

FILE(GLOB_RECURSE HEADER_LIST Client/*.hpp Libraries/*.hpp "External Libraries/*.hpp" "External Libraries/*.h" Server/*.hpp Shared/*.hpp)
SET(INCLUDES "")
FOREACH(FILE_PATH ${HEADER_LIST})
    GET_FILENAME_COMPONENT(DIR_PATH ${FILE_PATH} PATH)
    SET(INCLUDES ${INCLUDES} ${DIR_PATH})
ENDFOREACH()
LIST(REMOVE_DUPLICATES INCLUDES)

include_directories(Terralistic ${INCLUDES})
include_directories(Terralistic-server ${INCLUDES})
include_directories(Terralistic SYSTEM ${GLAD_INCLUDE_DIR} ${PF_INCLUDE_DIR})
include_directories(Terralistic-server SYSTEM ${GLAD_INCLUDE_DIR} ${PF_INCLUDE_DIR})

if(WIN32)
    SET(ZLIB_DIR ${CMAKE_SOURCE_DIR}/Dependencies/zlib-master/)
    SET(ZLIB_LIB zlib)
    SET(ZLIB_LIB_DIR ${ZLIB_DIR}lib/)
    SET(ZLIB_DLL ${ZLIB_DIR}bin/zlib.dll)
    SET(ZLIB_INCLUDE_DIR ${ZLIB_DIR}include/)

    SET(GLFW_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glfw-3.3.7.bin.WIN32/)
    SET(GLFW_DLL ${GLFW_DIR}lib-vc2022/glfw3.dll)
    SET(GLFW_LIB glfw3 opengl32)
    SET(GLFW_LIB_DIR ${GLFW_DIR}lib-vc2022/)
    SET(GLFW_INCLUDE_DIR ${GLFW_DIR}include/)

    target_link_directories(Terralistic PUBLIC ${ZLIB_LIB_DIR} ${GLFW_LIB_DIR})
    target_link_libraries(Terralistic Ws2_32.lib ${ZLIB_LIB} ${GLFW_LIB})

    target_link_directories(Terralistic-server PUBLIC ${ZLIB_LIB_DIR} ${GLFW_LIB_DIR})
    target_link_libraries(Terralistic-server Ws2_32.lib ${ZLIB_LIB} ${GLFW_LIB})

    include_directories(Terralistic ${ZLIB_INCLUDE_DIR}include/ ${GLFW_INCLUDE_DIR})

    file(COPY ${ZLIB_DLL} ${GLFW_DLL} DESTINATION .)
else()
    set(SFML_DIR ${CMAKE_SOURCE_DIR}/Dependencies/SFML-2.5.1/)
    set(GLFW_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glfw-3.3.7/)

    include_directories(${SFML_DIR}include/ ${GLFW_DIR}include/ ${CMAKE_SOURCE_DIR}/External\ Libraries/glad/include/)

    target_link_directories(Terralistic PUBLIC ${SFML_DIR}lib/ ${GLFW_DIR}build/src/)
    target_link_libraries(Terralistic z sfml-system sfml-network glfw3 X11)

    target_link_directories(Terralistic-server PUBLIC ${SFML_DIR}lib/ ${GLFW_DIR}build/src/)
    target_link_libraries(Terralistic-server z sfml-system sfml-network glfw3 X11)
endif()

file(COPY Build/Resources DESTINATION .)
