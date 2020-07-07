//
//  generatingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "terrainGenerator.hpp"
#include "framerateRegulator.hpp"

void terrainGenerator::generatingScreen() {
    bool running = true;
    SDL_Event event;

#define TEXT_SCALE 4
    
#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (swl::window_width / 5 * 4)
#define LOADING_RECT_ELAVATION 50
    
    ogl::texture loading_texture(ogl::centered);
    loading_texture.scale = TEXT_SCALE;
    loading_texture.setY((LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION) / 2);
    loading_texture.loadFromText("Generating world", SDL_Color{255, 255, 255});
    
    ogl::rect loading_rect(ogl::centered);
    loading_rect.setHeight(LOADING_RECT_HEIGHT);
    loading_rect.centered_y = false;
    
    short width = 0;
    
    while(running && loading_current < loading_total) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event))
            swl::handleBasicEvents(event, &running);
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        loading_rect.setWidth(LOADING_RECT_WIDTH);
        loading_rect.setX(0);
        loading_rect.setY(swl::window_height - LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION);
        loading_rect.setColor(100, 100, 100);
        loading_rect.render();
        
        width = (width * 3 + LOADING_RECT_WIDTH / (int)loading_total * (int)loading_current) / 4 + 1;
        loading_rect.setWidth(width);
        loading_rect.setX(width / 2 - LOADING_RECT_WIDTH / 2);
        loading_rect.setColor(255, 255, 255);
        loading_rect.render();
        
        loading_texture.render();
        
        swl::update();
    }
    if(!running) {
        swl::quit();
        exit(0);
    }
}
