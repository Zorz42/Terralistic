#include "mainMenu.hpp"
#include "fileManager.hpp"
#include "configManager.hpp"
#include "resourcePack.hpp"
#include "resourcePath.hpp"

int main(int argc, char **argv) {
    gfx::init(1000, 600);
    gfx::resource_path = getResourcePath(argv[0]);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    gfx::loadFont("pixel_font.ttf", 8);
    
    fileManager::init();
    {
        ConfigFile config(fileManager::getConfigPath());
        config.setDefaultInt("ui_scale", 100);
        gfx::setScale((float)config.getInt("ui_scale") / 100);
    }
    initProperties();
    
    MainMenu().run();

    gfx::quit();

    return 0;
}
