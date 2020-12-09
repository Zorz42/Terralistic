#include "gameLoop.hpp"
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "playerHandler.hpp"
#include "framerateRegulator.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "blockSelector.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "fileSystem.hpp"

#undef main

int gameLoop::main(std::string world_name) {
    blockEngine::prepare();
    if(fileSystem::dirExists(fileSystem::worlds_dir + world_name))
        worldSaver::loadWorld(world_name);
    else {
        terrainGenerator::generateTerrain(0);
        worldSaver::saveWorld(world_name);
    }
    
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
            if(swl::handleBasicEvents(event, &running) && !running)
                quit = true;
            else if(pauseScreen::handleEvents(event));
            else
                playerHandler::handleMovement(event);
            blockSelector::handleEvent(event);
        }
        playerHandler::doPhysics();
        playerHandler::move();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        
        playerHandler::render();
        blockEngine::render_blocks();
        if(pauseScreen::paused)
            pauseScreen::render();
        else {
            blockSelector::render();
            fps_text.render();
        }
        swl::update();
    }
    
    worldSaver::saveWorld(world_name);
    blockEngine::close();
    
    return 0;
}
