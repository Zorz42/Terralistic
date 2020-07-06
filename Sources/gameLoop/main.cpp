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
    
    ogl::texture fps_text(ogl::absolute);
    fps_text.setScale(3);
    fps_text.setX(10);
    fps_text.setY(10);
    
    bool running = true;
    SDL_Event event;
    
    unsigned int count = 0, fps_count = 0;
    
    while(running) {
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running));
            else
                playerHandler::handleMovement(event);
        }
        
        frameLengthMeasurer::measureFrameLength();
        fps_count++;
        if(SDL_GetTicks() / 1000 > count) {
            count++;
            fps_text.loadFromText(std::to_string(fps_count) + " fps", SDL_Color{0, 0, 0});
            fps_count = 0;
        }
        
        playerHandler::move();
        
        swl::setDrawColor(135, 206, 235);
        swl::clear();
        
        blockEngine::render_blocks();
        playerHandler::render();
        fps_text.render();
        
        swl::update();
    }
    
    return 0;
}
