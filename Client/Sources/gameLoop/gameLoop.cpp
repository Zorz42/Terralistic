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

gfx::sprite fps_text;

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
        networking::spawnListener();
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

void gameLoop::scene::onKeyUp(gfx::key key) {
    
}

void gameLoop::scene::onKeyDown(gfx::key key) {
    pauseScreen::onKeyDown(key);
}

void gameLoop::scene::update() {
    static unsigned int count = SDL_GetTicks() / 1000 - 1, fps_count = 0;
    fps_count++;
    if(SDL_GetTicks() / 1000 > count) {
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
    //itemRenderer::render();
    //players::render();
    //playerHandler::render();
    //blockSelector::render();
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
}

int gameLoop::main(const std::string& world_name, bool multiplayer) {
    /*online = multiplayer;
    running = true;
    
    blockEngine::prepare();
    players::prepare();
    blockRenderer::prepare();
    
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
    
    playerHandler::prepare();
    blockEngine::prepareWorld();
    
    ogl::texture fps_text(ogl::top_left);
    fps_text.scale = 3;
    fps_text.setX(10);
    fps_text.setY(10);
    
    SDL_Event event;
    
    unsigned int count = SDL_GetTicks() / 1000 - 1, fps_count = 0;*/
    
    while(running && main_::running) {
        //Uint64 start = SDL_GetPerformanceCounter();
        
        /*fps_count++;
        if(SDL_GetTicks() / 1000 > count) {
            count++;
            fps_text.loadFromText(std::to_string(fps_count) + " fps", SDL_Color{0, 0, 0});
            fps_count = 0;
        }*/
        
        /*while(SDL_PollEvent(&event)) { // <----------
            //SDL_StartTextInput();
            //swl::handleBasicEvents(event, &main_::running);
            pauseScreen::handleEvents(event);
            playerHandler::handleEvents(event);
            blockSelector::handleEvents(event);
        }*/
        
        /*playerHandler::doPhysics();
        playerHandler::move();
        if(!online) {
            itemEngine::updateItems(gfx::frame_length);
            playerHandler::lookForItems();
        }*/
        
        //swl::setDrawColor(135, 206, 235);
        //swl::clear();
        
        /*blockRenderer::render();
        itemRenderer::render();
        players::render();
        playerHandler::render();
        blockSelector::render();
        fps_text.render();
        pauseScreen::render();*/
        
        //swl::update();
        
        /*Uint64 end = SDL_GetPerformanceCounter();
        gfx::frame_length = float(end - start) / (float)SDL_GetPerformanceFrequency() * 1000.0f;*/
    }
    
    if(multiplayer)
        networking::sendPacket({packets::DISCONNECT});
    else
        worldSaver::saveWorld(world_name);
    blockEngine::close();
    itemEngine::close();
    blockRenderer::close();

    return 0;
}
