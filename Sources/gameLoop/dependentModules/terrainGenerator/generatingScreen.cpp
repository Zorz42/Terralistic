//
//  generatingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "terrainGenerator.hpp"

void terrainGenerator::generatingScreen() {
    bool running = true;
    SDL_Event event;

#define TEXT_SCALE 4
    
#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (swl::window_width / 5 * 4)
#define LOADING_RECT_ELAVATION 50
    
    SDL_Rect text_rect = {0, 0, 0, 0};
    SDL_Texture *text_texture = swl::loadTextureFromText("Generating world", SDL_Color{255, 255, 255}, &text_rect.w, &text_rect.h);
    text_rect.w *= TEXT_SCALE;
    text_rect.h *= TEXT_SCALE;
    
    SDL_Rect rect, outline_rect;
    while(running) {
        while(SDL_PollEvent(&event))
            swl::handleBasicEvents(event, &running);
            
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        outline_rect.w = LOADING_RECT_WIDTH;
        outline_rect.h = LOADING_RECT_HEIGHT;
        outline_rect.x = (swl::window_width - LOADING_RECT_WIDTH) / 2;
        outline_rect.y = swl::window_height - LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION;
        
        swl::setDrawColor(100, 100, 100);
        SDL_RenderFillRect(__swl_private::renderer, &outline_rect);
        
        rect.w = (rect.w * 3 + LOADING_RECT_WIDTH / (int)loading_total * (int)loading_current) / 4 + 1;
        rect.h = LOADING_RECT_HEIGHT;
        rect.x = (swl::window_width - LOADING_RECT_WIDTH) / 2;
        rect.y = swl::window_height - LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION;
        
        swl::setDrawColor(255, 255, 255);
        swl::render(rect);
        
        text_rect.y = (swl::window_height - LOADING_RECT_HEIGHT - LOADING_RECT_ELAVATION) / 2 - text_rect.h / 2;
        text_rect.x = swl::window_width / 2 - text_rect.w / 2;
        swl::render(text_texture, text_rect);
        
        swl::update();
        if(loading_current >= loading_total)
            break;
    }
    if(!running) {
        swl::quit();
        exit(0);
    }
}
