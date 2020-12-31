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
#include "itemEngine.hpp"
#include "inventoryRenderer.hpp"
#include "lightingEngine.hpp"

#undef main

int gameLoop::main(const std::string& world_name) {
    blockEngine::prepare();
    itemEngine::prepare();
    
    if(fileSystem::fileExists(fileSystem::worlds_dir + world_name + ".world"))
        worldSaver::loadWorld(world_name);
    else {
        for(itemEngine::inventoryItem& i : itemEngine::inventory) {
            i.setStack(0);
            i.item_id = itemEngine::NOTHING;
        }
        terrainGenerator::generateTerrain(0);
        worldSaver::saveWorld(world_name);
    }
    
    for(unsigned short x = 0; x < blockEngine::world_width; x++)
        lightingEngine::setNaturalLight(x);
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            blockEngine::getChunk(x, y).createTexture();
    
    ogl::texture fps_text(ogl::top_left);
    fps_text.scale = 3;
    fps_text.setX(10);
    fps_text.setY(10);
    
    running = true;
    SDL_Event event;
    
    unsigned int count = 0, fps_count = 0;
    
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
            else if(pauseScreen::handleEvents(event));
            else if(playerHandler::handleMovement(event));
            else if(itemEngine::handleEvents(event));
            else
                blockSelector::handleEvent(event);
        }
        playerHandler::doPhysics();
        playerHandler::move();
        itemEngine::updateItems();
        
        blockEngine::render_blocks();
        itemEngine::renderItems();
        playerHandler::render();
        if(pauseScreen::paused)
            pauseScreen::render();
        else {
            blockSelector::render();
            fps_text.render();
        }
        inventoryRenderer::render();
        swl::update();
    }
    
    worldSaver::saveWorld(world_name);
    blockEngine::close();
    itemEngine::close();

    return 0;
}
