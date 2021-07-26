//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include <thread>
#include <cassert>
#include <filesystem>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "textScreen.hpp"
#include "fileManager.hpp"
#include "choiceScreen.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"
#include "server.hpp"
#include "inventoryHandler.hpp"
#include "blockSelector.hpp"


#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;
static Server* private_server = nullptr;

#define TEXT_SCALE 3

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

void startPrivateWorld(const std::string& world_name) {
    gfx::Sprite loading_text;
    gfx::Rect loading_bar_back{0, -LOADING_RECT_ELEVATION, (unsigned short)(LOADING_RECT_WIDTH), LOADING_RECT_HEIGHT, {100, 100, 100}, gfx::BOTTOM},
    loading_bar{0, -LOADING_RECT_ELEVATION, 0, LOADING_RECT_HEIGHT, {255, 255, 255}, gfx::BOTTOM};

    loading_text.scale = TEXT_SCALE;
    loading_text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    loading_text.renderText("Generating world", {255, 255, 255});
    loading_text.orientation = gfx::CENTER;

    std::filesystem::create_directory(fileManager::getWorldsPath() + world_name);

    private_server = new Server(fileManager::getWorldsPath() + world_name, gfx::resource_path);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
    server_thread = std::thread(&Server::start, private_server, port);

    while(private_server->state != ServerState::RUNNING)
        switch (private_server->state) {
            case ServerState::NEUTRAL:
                gfx::sleep(1);
                break;
            case ServerState::STARTING:
                renderTextScreen("Starting server");
                break;
            case ServerState::LOADING_WORLD:
                renderTextScreen("Loading world");
                break;
            case ServerState::GENERATING_WORLD:
                loading_bar.w += (private_server->getGeneratingCurrent() * LOADING_RECT_WIDTH / private_server->getGeneratingTotal() - loading_bar.w) / 3;
                loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
                gfx::clearWindow();
                loading_text.render();
                loading_bar_back.render();
                loading_bar.render();
                gfx::updateWindow();
                break;
            default:
                assert(false);
                break;
        }

    private_server->setPrivate(true);

    game("_", "127.0.0.1", port).run();
    delete private_server;
    private_server = nullptr;
}

void game::init() {
    world_map = new map(&networking_manager);
    world_map->createWorld(275, 75); // dimensions in chunks

    InventoryHandler* inventory_handler = new InventoryHandler(&networking_manager);
    playerHandler* player_handler = new playerHandler(&networking_manager, &main_player, world_map);
    
    modules = {
        world_map,
        player_handler,
        new mapFront(world_map),
        new BlockSelector(world_map, &networking_manager, inventory_handler, player_handler),
        inventory_handler,
        new debugMenu(&main_player, world_map),
        new chat(&networking_manager),
        new pauseScreen(),
    };

    renderTextScreen("Connecting to server");
    if(!networking_manager.establishConnection(ip_address, port)) {
        choiceScreen("Could not connect to the server!", {"Close"}).run();
        gfx::returnFromScene();
    }
}

void game::update() {
    networking_manager.checkForPackets();
}

void game::stop() {
    networking_manager.closeConnection();
    
    if(private_server)
        private_server->stop();

    while(private_server && private_server->state != ServerState::STOPPING)
        renderTextScreen("Saving world");

    if(private_server) {
        server_thread.join();
        delete private_server;
        private_server = nullptr;
    }
}
