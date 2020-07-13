#include "gameLoop.hpp"
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "playerHandler.hpp"
#include "framerateRegulator.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "blockSelector.hpp"

int gameLoop::main() {
    blockEngine::init();
    playerHandler::init();
    terrainGenerator::generateTerrain(0);
    blockSelector::init();
    
    ogl::texture fps_text(ogl::top_left);
    fps_text.scale = 3;
    fps_text.setX(10);
    fps_text.setY(10);
    
    bool running = true;
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
            if(swl::handleBasicEvents(event, &running));
            else
                playerHandler::handleMovement(event);
            blockSelector::handleEvent(event);
        }
        
        playerHandler::move();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        
        blockEngine::render_blocks();
        playerHandler::render();
        blockSelector::render();
        
        fps_text.render();
        swl::update();
    }
    
    return 0;
}
