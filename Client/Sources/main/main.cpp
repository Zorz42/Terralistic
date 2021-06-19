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
#include <iostream>

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif

#else
#include "server.hpp"

#endif

#ifdef _WIN32
#define main Terralistic_main
int main(int argc, char **argv);
extern "C" int SDL_main(int argc, char **argv) {
    return main(argc, argv);
}
#endif

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    
    
    
    packets::packet test_packet(packets::PING, (int)std::string("it works!").size() + 1 + sizeof(int) + sizeof(unsigned int) + sizeof(short) + sizeof(unsigned short) + sizeof(char) + sizeof(unsigned char));
    test_packet << (int)5 << (unsigned int)6 << (short)7 << std::string("it works!") << (unsigned short)8 << (char)9 << (unsigned char)10;
    std::cout << (int)test_packet.get<unsigned char>() << std::endl;
    std::cout << (int)test_packet.get<char>() << std::endl;
    std::cout << (int)test_packet.get<unsigned short>() << std::endl;
    std::cout << test_packet.get<std::string>() << std::endl;
    std::cout << (int)test_packet.get<short>() << std::endl;
    std::cout << (int)test_packet.get<unsigned int>() << std::endl;
    std::cout << (int)test_packet.get<int>() << std::endl;
    
    int start = gfx::getTicks();
    for(int i = 0; i < 100000; i++) {
        //packets::packet test_packet(packets::PING, sizeof(int) + sizeof(int) + sizeof(int));
        //test_packet << 10 << 10 << 20;
    }
    std::cout << gfx::getTicks() - start << " ms" << std::endl;
    
    
    
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
