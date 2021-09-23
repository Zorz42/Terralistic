#include <filesystem>
#include <iostream>
#include "mainMenu.hpp"
#include "platform_folders.h"
#include "configManager.hpp"
#include "resourcePack.hpp"
#include "resourcePath.hpp"
#include "serverPlayers.hpp"
#include "settings.hpp"
#include "updater.hpp"
#include "versions.hpp"


int main(int argc, char **argv) {    
    srand(time(NULL));
    if(argc == 2 && (std::string)argv[1] == "version") {
        std::cout << CURR_VERSION_STR << std::endl;
        return 0;
    }
    
#ifndef DEVELOPER_MODE
    if(checkForUpdates(argv[0])) {
        system(((std::string)"\"" + argv[0] + "\"&").c_str());
        return 0;
    }
#endif
    gfx::init(1130, 700);
    gfx::resource_path = getResourcePath(argv[0]);
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    gfx::loadFont("pixel_font.ttf", 8);
    
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/");
    
    loadSettings();
    initProperties();
    
    MainMenu().run();

    gfx::quit();

    return 0;
}
