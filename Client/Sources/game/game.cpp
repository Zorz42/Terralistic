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
#include "itemRenderer.hpp"
#include "blockRenderer.hpp"

#undef main

static gfx::sprite fps_text;

INIT_SCRIPT
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
INIT_SCRIPT_END

void generateTerrain(unsigned int seed) {
    terrainGenerator::loading_total = 6;
    terrainGenerator::loading_current = 0;
    std::thread thread(terrainGenerator::generateTerrainDaemon, seed);
    
    terrainGenerator::generatingScreen();

    thread.join();
    
    ASSERT(terrainGenerator::loading_current == terrainGenerator::loading_total, "Loading total is " + std::to_string(terrainGenerator::loading_total) + ", but loading current got to " + std::to_string(terrainGenerator::loading_current));
}

void game::scene::init() {
    online = multiplayer;
    
    blockEngine::prepare();
    
    if(multiplayer) {
        if(!networking::establishConnection(world_name))
            gfx::returnFromScene();
        networking::startListening();
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory) {
            i.setStack(0);
            i.item_id = itemEngine::NOTHING;
        }
    } else if(fileSystem::fileExists(fileSystem::getWorldsPath() + world_name + ".world"))
        worldSaver::loadWorld(world_name);
    else {
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory)
            i.setStack(0);
        generateTerrain(0);
        worldSaver::saveWorld(world_name);
    }
    
    modules = {
        new blockRenderer::module(this),
        new itemRenderer::module(this),
        new players::module(this),
        new pauseScreen::module(this),
        new playerHandler::module(this),
        new blockSelector::module(this),
    };
}

void game::scene::onKeyDown(gfx::key key) {
}

void game::scene::onKeyUp(gfx::key key) {
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
        itemEngine::updateItems(gfx::frame_length);
}

void game::scene::render() {
    gfx::render(fps_text);
}

void game::scene::stop() {
    if(multiplayer)
        networking::sendPacket({packets::DISCONNECT});
    else
        worldSaver::saveWorld(world_name);
    
    networking::stopListening();
}
