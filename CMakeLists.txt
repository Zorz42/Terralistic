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

message(${CMAKE_CXX_COMPILER})

if(WIN32)
    SET(SFML_DIR ${CMAKE_SOURCE_DIR}/Dependencies/SFML-2.5.1/)
    SET(ZLIB_DIR ${CMAKE_SOURCE_DIR}/Dependencies/zlib/)
    SET(GLEW_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glew-2.1.0/)
    SET(GLFW_DIR ${CMAKE_SOURCE_DIR}/Dependencies/glfw-3.3.7.bin.WIN32/)

    set(SFML_LIBS_RELEASE sfml-system sfml-network sfml-main)
    set(SFML_LIBS_DEBUG sfml-system-d sfml-network-d sfml-main-d)

    SET(SFML_DLLS_RELEASE
            ${SFML_DIR}bin/sfml-network-2.dll
            ${SFML_DIR}bin/sfml-system-2.dll
            )

    SET(SFML_DLLS_DEBUG
            ${SFML_DIR}bin/sfml-network-d-2.dll
            ${SFML_DIR}bin/sfml-system-d-2.dll
            )

    SET(GLEW_DLL ${GLEW_DIR}bin/Release/Win32/glew32.dll)

    SET(GLFW_DLL ${GLFW_DIR}lib-vc2022/glfw3.dll)

    if(IS_DEBUG)
        SET(SFML_LIBS_TO_USE ${SFML_LIBS_DEBUG})
        SET(SFML_DLLS_TO_USE ${SFML_DLLS_DEBUG})
    else()
        SET(SFML_LIBS_TO_USE ${SFML_LIBS_RELEASE})
        SET(SFML_DLLS_TO_USE ${SFML_DLLS_RELEASE})
    endif()

    SET(ZLIB_LIB zlib)
    SET(ZLIB_DLL ${ZLIB_DIR}bin/zlib1.dll)

    SET(GLFW_LIB glfw3 opengl32)

    SET(GLEW_LIB glew32)

    target_link_directories(Terralistic PUBLIC ${SFML_DIR}lib/ ${ZLIB_DIR}lib/ ${GLFW_DIR}lib-vc2022/ ${GLEW_DIR}/lib/Release/Win32/)
    target_link_libraries(Terralistic Ws2_32.lib ${SFML_LIBS_TO_USE} ${ZLIB_LIB} ${GLFW_LIB} ${GLEW_LIB})

    target_link_directories(Terralistic-server PUBLIC ${SFML_DIR}lib/ ${ZLIB_DIR}lib/ ${GLFW_DIR}lib-vc2022/ ${GLEW_DIR}/lib/Release/Win32/)
    target_link_libraries(Terralistic-server Ws2_32.lib ${SFML_LIBS_TO_USE} ${ZLIB_LIB} ${GLFW_LIB} ${GLEW_LIB})

    include_directories(Terralistic ${SFML_DIR}include/ ${ZLIB_DIR}include/ ${GLEW_DIR}include/ ${GLFW_DIR}include/)

    file(COPY ${SFML_DLLS_TO_USE} ${ZLIB_DLL} ${GLFW_DLL} ${GLEW_DLL} DESTINATION .)
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
