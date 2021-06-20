//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include <thread>
#include <filesystem>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "otherPlayers.hpp"
#include "textScreen.hpp"
#include "fileManager.hpp"
#include "assert.hpp"
#include "choiceScreen.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif

#else
#include "server.hpp"

#endif

#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;
static server* private_server = nullptr;

#define TEXT_SCALE 3

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

void startPrivateWorld(const std::string& world_name) {
    gfx::sprite loading_text;
    gfx::rect loading_bar_back{0, -LOADING_RECT_ELEVATION, (unsigned short)(LOADING_RECT_WIDTH), LOADING_RECT_HEIGHT, {100, 100, 100}, gfx::bottom},
    loading_bar{0, -LOADING_RECT_ELEVATION, 0, LOADING_RECT_HEIGHT, {255, 255, 255}, gfx::bottom};

    loading_text.scale = TEXT_SCALE;
    loading_text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    loading_text.setTexture(gfx::renderText("Generating world", {255, 255, 255}));
    loading_text.orientation = gfx::center;

    std::filesystem::create_directory(fileManager::getWorldsPath() + world_name);

    private_server = new server(fileManager::getWorldsPath() + world_name, gfx::resource_path, rand() % (TO_PORT - FROM_PORT) + TO_PORT);
    server_thread = std::thread([ObjectPtr = private_server] { ObjectPtr->start(); });

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
                loading_bar.w += (private_server->getGeneratingCurrent() * LOADING_RECT_WIDTH / private_server->getGeneratingTotal() - loading_bar.w) / 3;
                loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
                gfx::clearWindow();
                gfx::render(loading_text);
                gfx::render(loading_bar_back);
                gfx::render(loading_bar);
                gfx::updateWindow();
                break;
            default:
                ASSERT(false, "Unregistered loading state!")
                break;
        }

    private_server->setPrivate(true);

    gfx::runScene(new game("_", "127.0.0.1", private_server->getPort()));
    delete private_server;
    private_server = nullptr;
}

void game::init() {
    world_map = new map(&networking_manager);
    world_map->createWorld(275, 75); // dimensions in chunks

    modules = {
        world_map,
        new players(&networking_manager, world_map),
        new playerHandler(&networking_manager, &main_player, world_map),
        new debugMenu(&main_player, world_map),
        new chat(&networking_manager),
        new pauseScreen(),
    };

    renderTextScreen("Connecting to server");
    if(!networking_manager.establishConnection(ip_address, port)) {
        gfx::runScene(new choiceScreen("Could not connect to the server!", {"Close"}));
        gfx::returnFromScene();
    }
}

void game::stop() {
    if(private_server)
        private_server->stop();

    while(private_server && private_server->state != server::STOPPING)
        renderTextScreen("Saving world");
    
    packets::packet disconnect_packet(packets::DISCONNECT, 0);
    networking_manager.sendPacket(disconnect_packet);
    networking_manager.closeConnection();

    if(private_server) {
        server_thread.join();
        delete private_server;
        private_server = nullptr;
    }
}
