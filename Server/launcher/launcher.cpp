#include <filesystem>
#include <thread>
#include <fstream>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"
#include "readOpa.hpp"

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
    
    resource_path = getResourcePath(argv[0]);
    
    if(gui) {
        gfx::init(800, 500, "Terralistic Server");
        gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
        
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
        ServerScene().run();
        main_server.stop();
        server_thread.join();
    } else {
        main_server.start();
    }
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
