#include <filesystem>
#include <thread>
#include <fstream>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"
#include "readOpa.hpp"
#include "launcherModule.hpp"
#include "worldInfo.hpp"
#include "console.hpp"
#include "moduleManager.hpp"
#include "playerInfo.hpp"

class ServerScene : public gfx::Scene {
    void init() override;
    void render() override;
public:
    ServerScene() : gfx::Scene("Server") {
        registerAModule((SceneModule*)new WorldInfo(resource_path));
        registerAModule((SceneModule*)new Console(resource_path));
        registerAModule((SceneModule*)new ModuleManager(resource_path));
        registerAModule((SceneModule*)new PlayerInfo(resource_path));
    }
};



int main(int argc, char **argv) {
    std::string data_folder = sago::getDataHome() + "/Terralistic-Server/";

    bool gui = true;

    if(argc > 1 && (std::string)argv[1] == "nogui")
        gui = false;

    if(!std::filesystem::exists(data_folder))
        std::filesystem::create_directory(data_folder);

    resource_path = getResourcePath(argv[0]);

    if(gui) {
        gfx::init(800, 500, "Terralistic Server");

        std::ifstream font_file(resource_path + "font.opa");
        std::vector<unsigned char> data = std::vector<unsigned char>((std::istreambuf_iterator<char>(font_file)), std::istreambuf_iterator<char>());
        int w = *(int*)&data[0];
        int h = *(int*)&data[sizeof(int)];
        data.erase(data.begin(), data.begin() + 8);

        gfx::Surface font_surface;
        font_surface.loadFromBuffer(data, w, h);
        gfx::loadFont(font_surface);
    }

    Server main_server(resource_path, data_folder + "world", 33770);

    if(gui) {
        std::thread server_thread(&Server::start, &main_server);
        auto scene = ServerScene();
        for(auto scene_module : scene.getModules()) {
            auto UI_module = (LauncherModule*) scene_module;
            UI_module->server = &main_server;
            if(*UI_module->getModuleName() == "module_manager"){
                auto console = (ModuleManager*)UI_module;
                console->module_vector = scene.getModules();
            }
        }
        scene.run();
        main_server.stop();
        server_thread.join();
    } else {
        main_server.start();
    }
}


void ServerScene::init() {
    int min_h = 0, min_w = 0;
    for(auto scene_module : getModules()){
        auto UI_module = (LauncherModule*) scene_module;
        min_w = std::max(min_w, UI_module->getMinWindowWidth());
        min_h = std::max(min_h, UI_module->getMinWindowHeight());
    }
    gfx::setMinimumWindowSize(min_w, min_h);//doesn't work?
}

void ServerScene::render() {
    gfx::RectShape(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight()).render(DARK_GREY);
    for(int i = 1; i < getModules().size(); i++) {//skiping server scene
        auto UI_module = (LauncherModule*) getModules()[i];
        if(!UI_module->enabled)
            continue;
        UI_module->base_container.w = (int)(UI_module->target_w * (float)gfx::getWindowWidth ());
        UI_module->base_container.h = (int)(UI_module->target_h * (float)gfx::getWindowHeight());
        UI_module->base_container.x = (int)(UI_module->target_x * (float)gfx::getWindowWidth ());
        UI_module->base_container.y = (int)(UI_module->target_y * (float)gfx::getWindowHeight());
    }
}

