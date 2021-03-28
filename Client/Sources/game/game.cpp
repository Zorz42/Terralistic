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
#include "networkingModule.hpp"
#include "otherPlayers.hpp"
#include "main.hpp"
#include "itemEngineClient.hpp"
#include "blockEngineClient.hpp"
#include "textScreen.hpp"

#undef main

static gfx::sprite fps_text;

INIT_SCRIPT
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
INIT_SCRIPT_END

void game::scene::init() {
    online = multiplayer;
    
    blockEngine::prepare();
    
    blockEngineClient::module* block_renderer = new blockEngineClient::module(this);
    itemEngineClient::module* item_engine = new itemEngineClient::module(this);
    players::module*              players = new players::module(this);
    pauseScreen::module*     pause_screen = new pauseScreen::module(this);
    playerHandler::module* player_handler = new playerHandler::module(this);
    blockSelector::module* block_selector = new blockSelector::module(this);
    
    networking_manager.listeners = {
        block_renderer,
        item_engine,
        players,
        player_handler,
    };
    
    modules = {
        block_renderer,
        item_engine,
        players,
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
