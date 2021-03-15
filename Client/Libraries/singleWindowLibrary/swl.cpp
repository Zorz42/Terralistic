//
//  swl.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#include <SDL2_image/SDL_image.h>
#include "singleWindowLibrary.hpp"

void swl::quit() {
    SDL_DestroyRenderer(swl_private::renderer); // assertion failure on close for some reason
    SDL_DestroyWindow(swl_private::window);
    SDL_Quit();
}

void swl::init() {
    swl::window_width = 1000;
    swl::window_height = 600;

    // initialize basic sdl module
    int result = SDL_Init(SDL_INIT_EVERYTHING);
    SDL_assert(result >= 0);

    // initialize image loading part of sdl
    result = IMG_Init(IMG_INIT_PNG);
    SDL_assert(result & IMG_INIT_PNG);
    
    // initialize font rendering part of sdl
    result = TTF_Init();
    SDL_assert(result != -1);
    
    // create actual window
    swl_private::window = SDL_CreateWindow("Terralistic", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, swl::window_width, swl::window_height, SDL_WINDOW_RESIZABLE);
    SDL_assert(swl_private::window);

    // create renderer for GPU accelerated
    swl_private::renderer = SDL_CreateRenderer(swl_private::window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    SDL_assert(swl_private::renderer);
    
    SDL_SetRenderDrawBlendMode(swl_private::renderer, SDL_BLENDMODE_BLEND);
    SDL_DisplayMode dm = {SDL_PIXELFORMAT_UNKNOWN, 0, 0, 0, nullptr};
    SDL_SetWindowDisplayMode(swl_private::window, &dm);
}

void swl::update() {
    // update screen / what has been rendered
    SDL_RenderPresent(swl_private::renderer);
}

void swl::clear() {
    // clear screen with current draw color
    SDL_RenderClear(swl_private::renderer);
}

bool swl::handleBasicEvents(SDL_Event &event, bool *running) {
    switch (event.type) {
        case SDL_QUIT:
            *running = false;
            return true;
        case SDL_WINDOWEVENT: // if window is resized, update window data
            if (event.window.event == SDL_WINDOWEVENT_RESIZED) {
                window_width = (unsigned short)event.window.data1;
                window_height = (unsigned short)event.window.data2;
                return true;
            }
            else
                return false;
        case SDL_MOUSEMOTION: // if mouse is moved, update mouse data
            SDL_GetMouseState((int*)&mouse_x, (int*)&mouse_y);
            return true;
        default:
            return false;
    }
}

bool swl::colliding(swl::rect a, swl::rect b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

void swl::setWindowMinimumSize(unsigned short width, unsigned short height) {
    SDL_SetWindowMinimumSize(swl_private::window, width, height);
}

void swl::setRenderTarget(SDL_Texture* texture) {
    // if you want to render to a texture
    SDL_SetRenderTarget(swl_private::renderer, texture);
}

void swl::resetRenderTarget() {
    setRenderTarget(nullptr);
}

SDL_Texture* swl::createBlankTexture(unsigned short width, unsigned short height) {
    SDL_Texture* result = SDL_CreateTexture(swl_private::renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, width, height);
    SDL_assert(result);
    return result;
}


