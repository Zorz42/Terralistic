//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include <thread>
#include <functional>
#include <filesystem>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "otherPlayers.hpp"
#include "textScreen.hpp"
#include "fileManager.hpp"
#include "assert.hpp"
#include "notifyingScreen.hpp"

#ifdef _WIN32
#include "server.hpp"
#else
#ifdef DEVELOPER_MODE
#include <Server_Debug/server.hpp>
#else
#include <Server/server.hpp>
#endif
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
    
    private_server = new server(fileManager::getWorldsPath() + world_name, rand() % (TO_PORT - FROM_PORT) + TO_PORT);
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
                loading_bar.w += (private_server->getGeneratingCurrent() * LOADING_RECT_WIDTH / private_server->getGeneratingTotal() - loading_bar.w) / 3;
                loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
                gfx::clearWindow();
                gfx::render(loading_text);
                gfx::render(loading_bar_back);
                gfx::render(loading_bar);
                gfx::updateWindow();
                break;
            default:
                ASSERT(false, "Unregistered loading state!");
                break;
        }
    
    private_server->setPrivate(true);
    
    gfx::runScene(new game("_", "127.0.0.1", private_server->getPort()));
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
    if(!networking_manager.establishConnection(ip_address, port)) {
        gfx::runScene(new notifyingScreen("Could not connect to the server!"));
        gfx::returnFromScene();
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
    if(debug_menu_open)
        gfx::render(fps_text);
}

void game::stop() {
    if(private_server)
        private_server->stop();
    
    while(private_server && private_server->state != server::STOPPING)
        renderTextScreen("Saving world");
    
    networking_manager.sendPacket(packets::DISCONNECT);
    networking_manager.closeConnection();
    
    if(private_server) {
        server_thread.join();
        delete private_server;
        private_server = nullptr;
    }
}

void game::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_M && !m_down) {
        m_down = true;
        debug_menu_open = !debug_menu_open;
    }
}

void game::onKeyUp(gfx::key key) {
    if(key == gfx::KEY_M)
        m_down = false;
}
