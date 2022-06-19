cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 20)
project(Terralistic-server CXX)
project(Terralistic CXX)

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

add_executable(Terralistic ${SRC_LIBRARIES} ${SRC_CLIENT} ${SRC_SERVER})
add_executable(Terralistic-server ${SRC_LIBRARIES} ${SRC_SERVER} ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp ${CMAKE_CURRENT_SOURCE_DIR})


FILE(GLOB_RECURSE HEADER_LIST Client/*.hpp Libraries/*.hpp "External Libraries/*.hpp" "External Libraries/*.h" Server/*.hpp Shared/*.hpp)
SET(INCLUDES "")
FOREACH(FILE_PATH ${HEADER_LIST})
    GET_FILENAME_COMPONENT(DIR_PATH ${FILE_PATH} PATH)
    SET(INCLUDES ${INCLUDES} ${DIR_PATH})
ENDFOREACH()
LIST(REMOVE_DUPLICATES INCLUDES)

include_directories(Terralistic ${INCLUDES})
include_directories(Terralistic-server ${INCLUDES})
include_directories(Terralistic SYSTEM "External Libraries/glad/include/")
include_directories(Terralistic-server SYSTEM "External Libraries/glad/include/")

message(${CMAKE_CXX_COMPILER})

if(WIN32)
    SET(ZLIB_DIR ${CMAKE_SOURCE_DIR}/Dependencies/zlib-master/)
    SET(GLFW_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glfw-3.3.7.bin.WIN32/)

    SET(GLFW_DLL ${GLFW_DIR}lib-vc2022/glfw3.dll)

    SET(ZLIB_LIB zlib)
    SET(ZLIB_DLL ${ZLIB_DIR}bin/zlib.dll)

    SET(GLFW_LIB glfw3 opengl32)

    target_link_directories(Terralistic PUBLIC ${ZLIB_DIR}lib/ ${GLFW_DIR}lib-vc2022/)
    target_link_libraries(Terralistic Ws2_32.lib ${ZLIB_LIB} ${GLFW_LIB})

    target_link_directories(Terralistic-server PUBLIC ${ZLIB_DIR}lib/ ${GLFW_DIR}lib-vc2022/)
    target_link_libraries(Terralistic-server Ws2_32.lib ${ZLIB_LIB} ${GLFW_LIB})

    include_directories(Terralistic ${ZLIB_DIR}include/ ${GLFW_DIR}include/)

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

file(COPY Resources DESTINATION .)
