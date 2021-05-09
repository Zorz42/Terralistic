//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include "startMenu.hpp"
#include "fileManager.hpp"
#include "playerHandler.hpp"

#ifdef _WIN32
#include "server.hpp"
#include <SDL.h>
#else
#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif
#endif

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::resource_path = fileManager::getResourcePath(argv[0]);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    serverInit();
    fileManager::init();
    map::initBlocks();
    map::initItems();
    playerHandler::initItems();
    
    gfx::switchScene(new startMenu());
    gfx::runScenes();
    
    gfx::quit();
    
    return 0;
}
