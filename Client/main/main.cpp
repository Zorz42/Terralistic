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
    srand((unsigned int)time(0));
    
    if(argc == 2 && (std::string)argv[1] == "version") {
        std::cout << CURR_VERSION_STR << std::endl;
        return 0;
    }
    
    gfx::init(getResourcePath(argv[0]), 1130, 700);
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    gfx::loadFont("pixel_font.ttf", 8);
    
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/");
    
    loadSettings();
    initProperties();
    
    MenuBack menu_back;
    menu_back.init();
    
#ifndef DEVELOPER_MODE
    UpdateChecker update_checker(&menu_back, argv[0]);
    update_checker.run();
    if(update_checker.hasUpdated()) {
        system(((std::string)"\"" + argv[0] + "\"&").c_str());
        gfx::quit();
        return 0;
    }
#endif
    MainMenu(&menu_back).run();

    gfx::quit();

    return 0;
}
