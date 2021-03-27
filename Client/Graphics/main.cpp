//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 08/03/2021.
//

#include "graphics-internal.hpp"

void gfx::init(unsigned short window_width_, unsigned short window_height_) {
    window_width = window_width_;
    window_height = window_height_;
    
    // initialize basic sdl module
    SDL_assert(SDL_Init(SDL_INIT_EVERYTHING) >= 0);

    // initialize image loading part of sdl
    SDL_assert(IMG_Init(IMG_INIT_PNG) & IMG_INIT_PNG);
    
    // initialize font rendering part of sdl
    SDL_assert(TTF_Init() != -1);
    
    // create actual window
    SDL_assert(window = SDL_CreateWindow("Terralistic", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, window_width, window_height, SDL_WINDOW_RESIZABLE));

    // create renderer for GPU accelerated
    SDL_assert(renderer = SDL_CreateRenderer(gfx::window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC));
    
    SDL_SetRenderDrawBlendMode(renderer, SDL_BLENDMODE_BLEND);
    SDL_DisplayMode dm = {SDL_PIXELFORMAT_UNKNOWN, 0, 0, 0, nullptr};
    SDL_SetWindowDisplayMode(window, &dm);
}

void gfx::setWindowMinimumSize(unsigned short width, unsigned short height) {
    SDL_SetWindowMinimumSize(window, width, height);
}

void gfx::loadFont(const std::string& path, unsigned char size) {
    font = TTF_OpenFont((resource_path + path).c_str(), size);
    SDL_assert(font);
}

void gfx::quit() {
    SDL_DestroyWindow(window);
    SDL_Quit();
}

unsigned short gfx::getWindowWidth() {
    return window_width;
}

unsigned short gfx::getWindowHeight() {
    return window_height;
}

unsigned short gfx::getMouseX() {
    return mouse_x;
}

unsigned short gfx::getMouseY() {
    return mouse_y;
}

void gfx::setRenderTarget(image& tex) {
    SDL_SetRenderTarget(renderer, (SDL_Texture*)tex.getTexture());
}

void gfx::resetRenderTarget() {
    SDL_SetRenderTarget(renderer, nullptr);
}

bool gfx::colliding(gfx::rectShape a, gfx::rectShape b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

unsigned int gfx::getTicks() {
    return SDL_GetTicks();
}

float gfx::getDeltaTime() {
    return frame_length;
}

void gfx::clearWindow() {
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
    SDL_RenderClear(renderer);
}

void gfx::updateWindow() {
    SDL_RenderPresent(renderer);
}
