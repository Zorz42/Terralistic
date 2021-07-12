#include "startMenu.hpp"
#include "fileManager.hpp"
#include "config.hpp"
#include "textures.hpp"

#ifdef _WIN32
#define main Terralistic_main
int main(int argc, char **argv);
extern "C" int SDL_main(int argc, char **argv) {
    return main(argc, argv);
}
#endif

int main(int argc, char **argv) {
    gfx::init(1000, 600);
    gfx::resource_path = fileManager::getResourcePath(argv[0]);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());

    fileManager::init();
    config = ConfigFile(fileManager::getDataPath() + "/config.txt");
    initProperties();
    loadTextures();
    
    gfx::runScene(new startMenu());

    gfx::quit();

    return 0;
}
