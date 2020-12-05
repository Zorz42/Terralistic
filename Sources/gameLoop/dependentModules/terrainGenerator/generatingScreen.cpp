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
#include "UIKit.hpp"

void terrainGenerator::generatingScreen() {
    bool running = true;
    SDL_Event event;

#define TEXT_SCALE 3
    
#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (swl::window_width / 5 * 4)
#define LOADING_RECT_ELAVATION 50
    
    ogl::texture loading_text(ogl::center);
    loading_text.scale = TEXT_SCALE;
    loading_text.setY((LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION) / 2);
    loading_text.loadFromText("Generating world", SDL_Color{255, 255, 255});
    
    ui::loadingBar loading_bar(loading_total, ogl::bottom);
    loading_bar.setHeight(LOADING_RECT_HEIGHT);
    loading_bar.setWidth(LOADING_RECT_WIDTH);
    loading_bar.setColor(255, 255, 255);
    loading_bar.setBackColor(100, 100, 100);
    loading_bar.setY(-LOADING_RECT_ELAVATION);
    loading_bar.bind(&loading_current);
    
    while(running && loading_current < loading_total) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event)) {
            swl::handleBasicEvents(event, &running);
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        loading_text.render();
        
        loading_bar.setWidth(LOADING_RECT_WIDTH);
        loading_bar.render();
        
        swl::update();
    }
    if(!running) {
        swl::quit();
        exit(0);
    }
}
