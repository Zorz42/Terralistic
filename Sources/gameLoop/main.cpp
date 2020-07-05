#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "playerHandler.hpp"
#include "frameLengthMeasurer.hpp"

#include "objectedGraphicsLibrary.hpp"

int main() {
    swl::loadFont("pixel_font.ttf", 8);
    
    blockEngine::init();
    playerHandler::init();
    terrainGenerator::generateTerrain(0);
    
    bool running = true;
    SDL_Event event;
    
    while(running) {
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running));
            else
                playerHandler::handleMovement(event);
        }
        
        playerHandler::move();
        
        frameLengthMeasurer::measureFrameLength();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        
        blockEngine::render_blocks();
        playerHandler::render();
        
        swl::update();
    }
    
    return 0;
}
