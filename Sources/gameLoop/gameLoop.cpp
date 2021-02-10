#include <thread>
#include "gameLoop.hpp"
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "playerHandler.hpp"
#include "framerateRegulator.hpp"
#include "blockSelector.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "fileSystem.hpp"
#include "inventory.hpp"
#include "generatingScreen.hpp"
#include "networkingModule.hpp"
#include "otherPlayers.hpp"

#undef main

void generateTerrain(unsigned int seed) {
    terrainGenerator::loading_total = 6;
    terrainGenerator::loading_current = 0;
    std::thread thread(terrainGenerator::generateTerrainDaemon, seed);
    
    terrainGenerator::generatingScreen();

    thread.join();
    
    if(terrainGenerator::loading_current != terrainGenerator::loading_total)
        swl::popupError("Loading total is " + std::to_string(terrainGenerator::loading_total) + ", but loading current got to " + std::to_string(terrainGenerator::loading_current));
}

int gameLoop::main(const std::string& world_name, bool multiplayer) {
    online = multiplayer;
    
    blockEngine::prepare();
    players::prepare();
    playerHandler::prepare();
    
    running = true;
    
    if(multiplayer) {
        if(!networking::establishConnection(world_name))
            return 0;
        networking::downloadWorld();
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
    
    while(running) {
        framerateRegulator::regulateFramerate();
        
        fps_count++;
        if(SDL_GetTicks() / 1000 > count) {
            count++;
            fps_text.loadFromText(std::to_string(fps_count) + " fps", SDL_Color{0, 0, 0});
            fps_count = 0;
        }
        
        while(SDL_PollEvent(&event)) {
            SDL_StartTextInput();
            if(swl::handleBasicEvents(event, &running) && !running)
                quit = true;
            pauseScreen::handleEvents(event);
            playerHandler::handleEvents(event);
            blockEngine::handleEvents(event);
        }
        playerHandler::doPhysics();
        playerHandler::move();
        if(!online)
            itemEngine::updateItems();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        blockEngine::render_blocks();
        itemEngine::renderItems();
        players::render();
        playerHandler::render();
        if(pauseScreen::paused)
            pauseScreen::render();
        else {
            blockSelector::render();
            fps_text.render();
        }
        swl::update();
    }
    
    if(multiplayer)
        networking::sendPacket({packets::DISCONNECT});
    else
        worldSaver::saveWorld(world_name);
    blockEngine::close();
    itemEngine::close();

    return 0;
}
