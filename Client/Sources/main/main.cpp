//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include "startMenu.hpp"
#include "fileManager.hpp"
#include "playerHandler.hpp"
#include "config.hpp"

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif

#else
#include "server.hpp"

#endif

/*#ifdef _WIN32
#define main Terralistic_main
int main(int argc, char **argv);
extern "C" int SDL_main(int argc, char **argv) {
    return main(argc, argv);
}
#endif*/

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::resource_path = fileManager::getResourcePath(argv[0]);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());

    serverInit();
    fileManager::init();
    config = configFile(fileManager::getDataPath() + "/config.txt");
    map::initBlocks();
    map::initItems();
    map::initLiquids();
    playerHandler::initItems();

    gfx::runScene(new startMenu());

    gfx::quit();

    return 0;
}
