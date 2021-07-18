#include "startMenu.hpp"
#include "fileManager.hpp"
#include "configManager.hpp"
#include "textures.hpp"
#include "resourcePath.hpp"

#ifdef _WIN32
#define main Terralistic_main
int main(int argc, char **argv);
extern "C" int SDL_main(int argc, char **argv) {
    return main(argc, argv);
}
#endif

int main(int argc, char **argv) {
    
    gfx::init(1000, 600);
    gfx::resource_path = getResourcePath(argv[0]);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    fileManager::init();
    {
        ConfigFile config(fileManager::getDataPath() + "/config.txt");
        config.setDefaultInt("ui_scale", 100);
        gfx::setScale((float)config.getInt("ui_scale") / 100);
    }
    initProperties();
    loadTextures();
    
    startMenu().run();

    gfx::quit();

    return 0;
}
