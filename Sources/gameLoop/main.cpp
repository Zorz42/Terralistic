//Using SDL and standard IO
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "movementHandler.hpp"
#include "frameLengthMeasurer.hpp"

int main() {
    swl::loadFont("pixel_font.ttf", 8);
    
    block_engine::init();
    terrainGenerator::generateTerrain(0);
    
    bool running = true;
    SDL_Event event;
    
    while(running) {
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running));
            else
                movementHandler::handleMovement(event);
        }
        
        movementHandler::move();
        frameLengthMeasurer::measureFrameLength();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        
        block_engine::render_blocks();

        swl::update();
    }
    
    return 0;
}
