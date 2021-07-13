cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 17)
project(Terralistic)
project(Terralistic-server)

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(IS_DEBUG TRUE)
    set(IS_RELEASE FALSE)
else()
    set(IS_DEBUG FALSE)
    set(IS_RELEASE TRUE)
endif()


# settings
if(IS_DEBUG)
    add_compile_definitions(DEVELOPER_MODE)
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Ofast")
endif()

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread -I/usr/include/SDL2")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")
endif()

# set sources
file(GLOB_RECURSE SRC_LIBRARIES CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Libraries/*.cpp)
file(GLOB_RECURSE SRC_CLIENT CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Client/*.cpp)
file(GLOB_RECURSE SRC_SERVER CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Server/*.cpp)
list(REMOVE_ITEM SRC_SERVER ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)

add_executable(Terralistic WIN32 ${SRC_LIBRARIES} ${SRC_CLIENT} ${SRC_SERVER})
add_executable(Terralistic-server ${SRC_LIBRARIES} ${SRC_SERVER} ${CMAKE_CURRENT_SOURCE_DIR}/Server/launcher/launcher.cpp)


# set includes
FILE(GLOB_RECURSE HEADER_LIST *.hpp *.h)
SET(INCLUDES "")
FOREACH(FILE_PATH ${HEADER_LIST})
    GET_FILENAME_COMPONENT(DIR_PATH ${FILE_PATH} PATH)
    SET(INCLUDES ${INCLUDES} ${DIR_PATH})
ENDFOREACH()
LIST(REMOVE_DUPLICATES INCLUDES)

include_directories(Terralistic ${INCLUDES})
include_directories(Terralistic-server ${INCLUDES})

# link libs
if(WIN32)
    SET(SFML_DIR ${CMAKE_SOURCE_DIR}/Dependencies/SFML-2.5.1/)

    SET(SFML_EXTLIBS
            ${SFML_DIR}lib/flac.lib
            ${SFML_DIR}lib/freetype.lib
            ${SFML_DIR}lib/ogg.lib
            ${SFML_DIR}lib/openal32.lib
            ${SFML_DIR}lib/vorbis.lib
            ${SFML_DIR}lib/vorbisenc.lib
            ${SFML_DIR}lib/vorbisfile.lib
            )

    SET(SFML_LIBS_RELEASE
            ${SFML_EXTLIBS}
            ${SFML_DIR}lib/sfml-audio.lib
            ${SFML_DIR}lib/sfml-graphics.lib
            ${SFML_DIR}lib/sfml-main.lib
            ${SFML_DIR}lib/sfml-network.lib
            ${SFML_DIR}lib/sfml-system.lib
            ${SFML_DIR}lib/sfml-window.lib
            )

    SET(SFML_LIBS_DEBUG
            ${SFML_EXTLIBS}
            ${SFML_DIR}lib/sfml-audio-d.lib
            ${SFML_DIR}lib/sfml-graphics-d.lib
            ${SFML_DIR}lib/sfml-main-d.lib
            ${SFML_DIR}lib/sfml-network-d.lib
            ${SFML_DIR}lib/sfml-system-d.lib
            ${SFML_DIR}lib/sfml-window-d.lib
            )

    SET(SFML_DLLS_RELEASE
            ${SFML_DIR}bin/openal32.dll
            ${SFML_DIR}bin/sfml-audio-2.dll
            ${SFML_DIR}bin/sfml-graphics-2.dll
            ${SFML_DIR}bin/sfml-network-2.dll
            ${SFML_DIR}bin/sfml-system-2.dll
            ${SFML_DIR}bin/sfml-window-2.dll
            )
    SET(SFML_DLLS_DEBUG
            ${SFML_DIR}bin/openal32.dll
            ${SFML_DIR}bin/sfml-audio-d-2.dll
            ${SFML_DIR}bin/sfml-graphics-d-2.dll
            ${SFML_DIR}bin/sfml-network-d-2.dll
            ${SFML_DIR}bin/sfml-system-d-2.dll
            ${SFML_DIR}bin/sfml-window-d-2.dll
            )

    if(IS_DEBUG)
        SET(SFML_LIBS_TO_USE ${SFML_LIBS_DEBUG})
        SET(SFML_DLLS_TO_USE ${SFML_DLLS_DEBUG})
    else()
        SET(SFML_LIBS_TO_USE ${SFML_LIBS_RELEASE})
        SET(SFML_DLLS_TO_USE ${SFML_DLLS_RELEASE})
    endif()

    target_link_libraries(Terralistic
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2-2.0.14/lib/x64/SDL2main.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2_image-2.0.5/lib/x64/SDL2_image.lib
            ${CMAKE_SOURCE_DIR}/Dependencies/Windows/SDL2_ttf-2.0.15/lib/x64/SDL2_ttf.lib
            Ws2_32.lib
            ${SFML_LIBS_TO_USE}
            )

    target_link_libraries(Terralistic-server
            Ws2_32.lib
            ${SFML_LIBS_TO_USE}
            )

    include_directories(Terralistic ${SFML_DIR}include/)

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

            ${SFML_DLLS_TO_USE}

            DESTINATION .)
else()
    target_link_libraries(Terralistic SDL2 SDL2_ttf SDL2_image sfml-system sfml-graphics sfml-audio sfml-network sfml-window)
endif()

file(COPY Client/Resources DESTINATION .)
file(COPY Client/Resources/Structures.asset DESTINATION .)
