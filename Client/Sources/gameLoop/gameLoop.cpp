//
//  gameLoop.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include "core.hpp"

#include "gameLoop.hpp"
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
static bool running;

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

void gameLoop::scene::init() {
    online = multiplayer;
    running = true;
    
    blockEngine::prepare();
    players::prepare();
    blockRenderer::prepare();
    
    if(multiplayer) {
        if(!networking::establishConnection(world_name))
            gfx::returnFromScene();
        networking::startListening();
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory) {
            i.setStack(0);
            i.item_id = itemEngine::NOTHING;
        }
    } else if(fileSystem::fileExists(fileSystem::worlds_dir + world_name + ".world"))
        worldSaver::loadWorld(world_name);
    else {
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory)
            i.setStack(0);
        generateTerrain(0);
        worldSaver::saveWorld(world_name);
    }
    
    playerHandler::prepare();
    blockEngine::prepareWorld();
}

void gameLoop::scene::onKeyDown(gfx::key key) {
    pauseScreen::onKeyDown(key);
    playerHandler::onKeyDown(key);
    blockSelector::onKeyDown(key);
}

void gameLoop::scene::onKeyUp(gfx::key key) {
    playerHandler::onKeyUp(key);
    blockSelector::onKeyUp(key);
}

void gameLoop::scene::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1, fps_count = 0;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        fps_text.setTexture(gfx::renderText(std::to_string(fps_count) + " fps", {0, 0, 0}));
        fps_count = 0;
    }
    
    playerHandler::doPhysics();
    playerHandler::move();
    if(!online) {
        itemEngine::updateItems(gfx::frame_length);
        playerHandler::lookForItems();
    }
}

void gameLoop::scene::render() {
    blockRenderer::render();
    itemRenderer::render();
    players::render();
    playerHandler::render();
    blockSelector::render();
    gfx::render(fps_text);
    pauseScreen::render();
}

void gameLoop::scene::stop() {
    if(multiplayer)
        networking::sendPacket({packets::DISCONNECT});
    else
        worldSaver::saveWorld(world_name);
    blockEngine::close();
    itemEngine::close();
    blockRenderer::close();
    networking::stopListening();
}
