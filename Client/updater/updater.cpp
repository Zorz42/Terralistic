#include <SFML/Network.hpp>
#include <iostream>
#include <fstream>
#include <filesystem>
#include "updater.hpp"
#include "versions.hpp"
#include "platform_folders.h"

#ifdef __APPLE__
#define PATCH_REQEUST_VERSION (std::string("MacOS-") + CURR_VERSION_STR)
#endif

#ifdef _WIN32
#define PATCH_REQEUST_VERSION (std::string("Windows-") + CURR_VERSION_STR)
#endif

#ifdef __linux__
#define PATCH_REQEUST_VERSION (std::string("Linux-") + CURR_VERSION_STR)
#endif

void UpdateChecker::checkForUpdates() {
    update_state = UpdateState::CHECKING;
    sf::TcpSocket server_socket;
    if(server_socket.connect("jakob.zorz.si", 65431) == sf::Socket::Done) {
        server_socket.send(PATCH_REQEUST_VERSION.c_str(), PATCH_REQEUST_VERSION.size());

        std::string buffer;
        char temp_buffer[1024];
        std::size_t received;
        while(server_socket.receive(temp_buffer, 1024, received) == sf::Socket::Done) {
            update_state = UpdateState::DOWNLOADING;
            buffer.insert(buffer.end(), &temp_buffer[0], &temp_buffer[0] + received);
        }
        
        if(!buffer.empty()) {
            std::ofstream patch_file(sago::getDataHome() + "/Terralistic/update.patch", std::ios::binary | std::ios::trunc);
            patch_file.write(&buffer[0], buffer.size());
        }
    }
    
    while(exec_path.back() != '\\' && exec_path.back() != '/')
        exec_path.pop_back();
    
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/update.patch")) {
        update_state = UpdateState::APPLYING;
#ifdef __APPLE__
        std::system(((std::string)"cd \"" + exec_path + "../..\" && patch -t -p3 < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
#endif

#ifdef _WIN32
        std::system(((std::string)"cd \"" + exec_path + "\" && patch -t -p3 --binary < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
#endif

#ifdef __linux__
        std::system(((std::string)"cd \"" + exec_path + "\" && patch -t -p3 < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
#endif
        std::filesystem::remove(sago::getDataHome() + "/Terralistic/update.patch");
        has_updated = true;
    }
    update_state = UpdateState::FINISHED;
}

void UpdateChecker::init() {
    update_thread = std::thread(&UpdateChecker::checkForUpdates, this);
    
    text.scale = 3;
    text.orientation = gfx::CENTER;
    text.renderText("Checking for updates");
}

void UpdateChecker::render() {
    static UpdateState prev_update_state = UpdateState::NEUTRAL;
    menu_back->setBackWidth(text.getWidth() + 100);
    menu_back->renderBack();
    
    if(update_state != prev_update_state) {
        prev_update_state = update_state;
        switch (update_state) {
            case UpdateState::NEUTRAL:
                break;
            case UpdateState::CHECKING:
                text.renderText("Checking for updates");
                break;
            case UpdateState::DOWNLOADING:
                text.renderText("Downloading updates");
                break;
            case UpdateState::APPLYING:
                text.renderText("Applying updates");
                break;
            case UpdateState::FINISHED:
                update_thread.join();
                gfx::returnFromScene();
                break;
        }
    }
    
    text.render();
}