//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include <thread>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "generatingScreen.hpp"
#include "otherPlayers.hpp"
#include "textScreen.hpp"
#include "fileManager.hpp"
#include "fileSystem.hpp"
#include "assert.hpp"

#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif

static std::thread server_thread;
static server* private_server = nullptr;

void startPrivateWorld(const std::string& world_name) {
    fileSystem::createDirIfNotExists(fileManager::getWorldsPath() + world_name);
    
    private_server = new server(fileManager::getWorldsPath() + world_name);
    server_thread = std::thread(std::bind(&server::start, private_server));
    
    while(private_server->state != server::RUNNING)
        switch (private_server->state) {
            case server::NEUTRAL:
                gfx::sleep(1);
                break;
            case server::STARTING:
                renderTextScreen("Starting server");
                break;
            case server::LOADING_WORLD:
                renderTextScreen("Loading world");
                break;
            case server::GENERATING_WORLD:
                renderTextScreen("Generating world");
                break;
            default:
                ASSERT(false, "Unregistered loading state!");
                break;
        }
    
    gfx::switchScene(new game("127.0.0.1"));
}

void game::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
    
    world_map = new map(&networking_manager);
    world_map->createWorld(275, 75); // dimensions in chunks
    
    modules = {
        world_map,
        new players(&networking_manager, world_map),
        new pauseScreen(),
        new playerHandler(&networking_manager, &main_player, world_map),
    };
    
    renderTextScreen("Connecting to server");
    if(!networking_manager.establishConnection(ip_address)) {
        gfx::returnFromScene();
        return;
    }
}

void game::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1, fps_count = 0;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        fps_text.setTexture(gfx::renderText(std::to_string(fps_count) + " fps", {0, 0, 0}));
        fps_count = 0;
    }
}

void game::render() {
    gfx::render(fps_text);
}

void game::stop() {
    private_server->stop();
    
    while (private_server->state != server::STOPPING)
        gfx::sleep(1);
    
    networking_manager.sendPacket(packets::DISCONNECT);
    networking_manager.closeConnection();
    
    if(private_server) {
        server_thread.join();
        delete private_server;
        private_server = nullptr;
    }
}
