#include <filesystem>
#include <thread>
#include <fstream>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"
#include "readOpa.hpp"
#include "launcherModule.hpp"
#include "worldInfo.hpp"
#include "configManager.hpp"

class ServerScene : public gfx::Scene {
    void init() override;
    void render() override;
public:
    std::vector<LauncherModule*> modules;
    ServerScene() : gfx::Scene("Server") {
        ConfigFile file(resource_path + "resourcePack/userinterface/server_ui.config");
        std::string temp_str_modules = file.getStr("modules");
        std::vector<int> activated_modules;
        size_t pos;
        while ((pos = temp_str_modules.find(' ')) != std::string::npos) {
            activated_modules.push_back(std::stoi(temp_str_modules.substr(0, pos)));
            temp_str_modules.erase(0, pos + 1);
        }
        activated_modules.push_back(std::stoi(temp_str_modules));
        for(int i = 0; i < activated_modules.size(); i++){
            std::string properties = file.getStr(std::to_string(i));
            int nums[8];
            for(int j = 0; j < 4; j++){
                pos = properties.find('/');
                nums[2 * j] = std::stoi(properties.substr(0, pos));
                properties.erase(0, pos + 1);
                pos = properties.find(' ');
                nums[2 * j + 1] = std::stoi(properties.substr(0, pos));
                properties.erase(0, pos + 1);
            }
            float x = (float)nums[0] / (float)nums[1];
            float y = (float)nums[2] / (float)nums[3];
            float w = (float)nums[4] / (float)nums[5];
            float h = (float)nums[6] / (float)nums[7];
            if(x + w > 1 || y + h > 1 || w == 0 || h == 0)
                continue;
            switch (activated_modules[i]) {
                case 1:{
                    modules.push_back((LauncherModule*)new WorldInfo(x, y, w, h));
                    break;
                }
                default:
                    continue;
            }
        }
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
        for(auto UImodule : scene.modules) {
            UImodule->server = &main_server;
            UImodule->init();
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
    for(auto module : modules){
        min_w = std::max(min_w, module->getMinWindowWidth());
        min_h = std::max(min_h, module->getMinWindowWidth());
    }
    gfx::setMinimumWindowSize(min_w, min_h);//doesn't work?
}

void ServerScene::render() {
    gfx::RectShape(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight()).render(BLACK);
    for(auto module : modules) {
        module->width = (int)(module->target_w * (float)gfx::getWindowWidth());
        module->height = (int)(module->target_h * (float)gfx::getWindowHeight());
        module->render();
        module->texture.render(1, ((int)(module->target_x * (float)gfx::getWindowWidth())), ((int)(module->target_y * (float)gfx::getWindowHeight())));
    }
}
