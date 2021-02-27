//
//  gameLoop.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//
#define FILENAME gameLoop
#define NAMESPACE gameLoop
#include "core.hpp"

#include "gameLoop.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"
#include "blockSelector.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "generatingScreen.hpp"
#include "networkingModule.hpp"
#include "otherPlayers.hpp"
#include "main.hpp"
#include "renderer.hpp"
#include "blockRenderer.hpp"

#undef main

void generateTerrain(unsigned int seed) {
    terrainGenerator::loading_total = 6;
    terrainGenerator::loading_current = 0;
    std::thread thread(terrainGenerator::generateTerrainDaemon, seed);
    
    terrainGenerator::generatingScreen();

    thread.join();
    
    ASSERT(terrainGenerator::loading_current == terrainGenerator::loading_total, "Loading total is " + std::to_string(terrainGenerator::loading_total) + ", but loading current got to " + std::to_string(terrainGenerator::loading_current));
}

int gameLoop::main(const std::string& world_name, bool multiplayer) {
    online = multiplayer;
    running = true;
    
    blockEngine::prepare();
    players::prepare();
    playerHandler::prepare();
    
    if(multiplayer) {
        if(!networking::establishConnection(world_name))
            return 0;
        networking::spawnListener();
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory) {
            i.setStack(0);
            i.item_id = itemEngine::NOTHING;
        }
    } else if(fileSystem::fileExists(fileSystem::worlds_dir + world_name + ".world"))
        worldSaver::loadWorld(world_name);
    else {
        for(inventory::inventoryItem& i : playerHandler::player_inventory.inventory) {
            i.setStack(0);
            i.item_id = itemEngine::NOTHING;
        }
        generateTerrain(0);
        worldSaver::saveWorld(world_name);
    }
    
    
    
    blockEngine::prepareWorld();
    
    ogl::texture fps_text(ogl::top_left);
    fps_text.scale = 3;
    fps_text.setX(10);
    fps_text.setY(10);
    
    SDL_Event event;
    
    unsigned int count = SDL_GetTicks() / 1000 - 1, fps_count = 0;
    
    while(running && main_::running) {
        Uint64 start = SDL_GetPerformanceCounter();
        
        fps_count++;
        if(SDL_GetTicks() / 1000 > count) {
            count++;
            fps_text.loadFromText(std::to_string(fps_count) + " fps", SDL_Color{0, 0, 0});
            fps_count = 0;
        }
        
        while(SDL_PollEvent(&event)) {
            SDL_StartTextInput();
            swl::handleBasicEvents(event, &main_::running);
            pauseScreen::handleEvents(event);
            playerHandler::handleEvents(event);
            blockSelector::handleEvents(event);
        }
        playerHandler::doPhysics();
        playerHandler::move();
        if(!online) {
            itemEngine::updateItems(frame_length);
            playerHandler::lookForItems();
        }
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        blockRenderer::render();
        renderer::renderItems();
        players::render();
        playerHandler::render();
        if(pauseScreen::paused)
            pauseScreen::render();
        else {
            blockSelector::render();
            fps_text.render();
        }
        swl::update();
        
        Uint64 end = SDL_GetPerformanceCounter();
        frame_length = float(end - start) / (float)SDL_GetPerformanceFrequency() * 1000.0f;
    }
    
    if(multiplayer)
        networking::sendPacket({packets::DISCONNECT});
    else
        worldSaver::saveWorld(world_name);
    blockEngine::close();
    itemEngine::close();

    return 0;
}
