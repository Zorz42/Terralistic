//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include "core.hpp"

#include "game.hpp"
#include "playerHandler.hpp"
#include "blockSelector.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "generatingScreen.hpp"
#include "otherPlayers.hpp"
#include "itemEngineClient.hpp"
#include "blockRenderer.hpp"
#include "textScreen.hpp"

#undef main

void game::scene::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
    
    online = multiplayer;
    
    blockEngine::prepare();
    
    auto* block_renderer = new blockRenderer(this);
    auto* item_engine    = new itemEngineClient::module(this);
    auto* player         = new players::module(this);
    auto* pause_screen   = new pauseScreen::module(this);
    auto* player_handler = new playerHandler::module(this);
    auto* block_selector = new blockSelector::module(this);
    
    networking_manager.listeners = {
        block_renderer,
        item_engine,
        player,
        player_handler,
    };
    
    modules = {
        block_renderer,
        item_engine,
        player,
        pause_screen,
        player_handler,
        block_selector,
    };
    
    if(multiplayer) {
        textScreen::renderTextScreen("Connecting to server");
        if(!networking_manager.startListening(world_name)) {
            gfx::returnFromScene();
            return;
        }
        playerHandler::player_inventory.clear();
    } else if(fileSystem::fileExists(fileSystem::getWorldsPath() + world_name + ".world")) {
        textScreen::renderTextScreen("Loading world");
        worldSaver::loadWorld(world_name);
    }
    else {
        playerHandler::player_inventory.clear();
        gfx::switchScene(new terrainGenerator::scene(0));
    }
}

void game::scene::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1, fps_count = 0;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        fps_text.setTexture(gfx::renderText(std::to_string(fps_count) + " fps", {0, 0, 0}));
        fps_count = 0;
    }
    
    if(!online)
        itemEngine::updateItems(gfx::getDeltaTime());
}

void game::scene::render() {
    gfx::render(fps_text);
}

void game::scene::stop() {
    if(multiplayer)
        networking_manager.sendPacket({packets::DISCONNECT});
    else {
        textScreen::renderTextScreen("Saving world");
        worldSaver::saveWorld(world_name);
    }
    
    networking_manager.stopListening();
}
