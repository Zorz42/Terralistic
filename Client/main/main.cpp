#include <filesystem>
#include "mainMenu.hpp"
#include "platform_folders.h"
#include "configManager.hpp"
#include "resourcePack.hpp"
#include "resourcePath.hpp"
#include "serverPlayers.hpp"
#include "settings.hpp"

int main(int argc, char **argv) {
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
