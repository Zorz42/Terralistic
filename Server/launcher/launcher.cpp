#include <filesystem>
#include <thread>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"
#include "serverPlayers.hpp"
#include "graphics.hpp"

class ServerScene : public gfx::Scene {
    gfx::Sprite text;
    void init() override;
    void render() override;
public:
};

int main(int argc, char **argv) {
    std::string data_folder = sago::getDataHome() + "/Terralistic-Server/";
    
    if(!std::filesystem::exists(data_folder))
        std::filesystem::create_directory(data_folder);
    
    gfx::init(getResourcePath(argv[0]), 500, 300);
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    gfx::loadFont("pixel_font.ttf", 8);
    
    initProperties();
    
    Server main_server(gfx::getResourcePath(), data_folder + "world", 33770);
    
    std::thread server_thread(&Server::start, &main_server);
    
    ServerScene().run();
    
    main_server.stop();
    server_thread.join();
}

void ServerScene::init() {
    text.scale = 3;
    text.orientation = gfx::CENTER;
    text.loadFromText("Server Running");
}

void ServerScene::render() {
    gfx::RectShape(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight()).render(BLACK);
    text.render();
}
