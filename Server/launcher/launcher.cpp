#include <filesystem>
#include <thread>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"

class ServerScene : public gfx::Scene {
    gfx::Sprite text;
    void init() override;
    void render() override;
public:
};

int main(int argc, char **argv) {
    std::string data_folder = sago::getDataHome() + "/Terralistic-Server/";
    
    bool gui = true;
    
    if(argc > 1 && (std::string)argv[1] == "nogui")
        gui = false;
    
    if(!std::filesystem::exists(data_folder))
        std::filesystem::create_directory(data_folder);
    
    std::string resource_path = getResourcePath(argv[0]);
    
    if(gui) {
        gfx::init(resource_path, 500, 300);
        gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
        gfx::loadFont("font.ttf", 16);
    }
    
    Server main_server(resource_path, data_folder + "world", 33770);
    
    if(gui) {
        std::thread server_thread(&Server::start, &main_server);
        ServerScene().run();
        main_server.stop();
        server_thread.join();
    } else {
        main_server.start();
    }
}

void ServerScene::init() {
    text.scale = 4;
    text.orientation = gfx::CENTER;
    text.loadFromText("Server Running");
}

void ServerScene::render() {
    gfx::RectShape(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight()).render(BLACK);
    text.render();
}
