//
//  generatingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "terrainGenerator.hpp"
#include "generatingScreen.hpp"
#include "UIKit.hpp"
#include "gameLoop.hpp"
#include "main.hpp"

void terrainGenerator::generatingScreen() {
    // puts generation of world into another thread and only renders progress bar and text
    
    bool running = true;
    SDL_Event event;

#define TEXT_SCALE 3
    
#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (swl::window_width / 5 * 4)
#define LOADING_RECT_ELEVATION 50
    
    ogl::texture loading_text(ogl::center);
    loading_text.scale = TEXT_SCALE;
    loading_text.setY((LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2);
    loading_text.loadFromText("Generating world", SDL_Color{255, 255, 255});
    
    ui::loadingBar loading_bar((unsigned short)loading_total, ogl::bottom);
    loading_bar.setHeight(LOADING_RECT_HEIGHT);
    loading_bar.setWidth((unsigned short)LOADING_RECT_WIDTH);
    loading_bar.setColor(255, 255, 255);
    loading_bar.setBackColor(100, 100, 100);
    loading_bar.setY(-LOADING_RECT_ELEVATION);
    loading_bar.bind(&loading_current);
    
    while(running && loading_current < loading_total && main_::running) {
        while(SDL_PollEvent(&event)) {
            swl::handleBasicEvents(event, &main_::running);
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        loading_text.render();
        
        loading_bar.setWidth((unsigned short)LOADING_RECT_WIDTH);
        loading_bar.render();
        
        swl::update();
    }
}
