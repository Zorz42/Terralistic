//
//  swl.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#include <SDL2_image/SDL_image.h>
#include "singleWindowLibrary.hpp"

void swl::quit() {
    //SDL_DestroyRenderer(swl_private::renderer); // assertion failure on close for some reason
    SDL_DestroyWindow(swl_private::window);
    SDL_Quit();
}

//#undef main

int main([[maybe_unused]] int argc, char **argv) {
    swl::window_width = 1000;
    swl::window_height = 600;
    
    if(SDL_Init(SDL_INIT_EVERYTHING) < 0)
        swl::popupError("SDL could not initialize properly!");

    int img_flags = IMG_INIT_PNG;
    if(!(IMG_Init(img_flags) & img_flags))
        swl::popupError("SDL_image could not initialize properly!");
    
    if(TTF_Init() == -1)
        swl::popupError("SDL_ttf could not initialize properly!");
    
    swl_private::window = SDL_CreateWindow("Terralistic", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, swl::window_width, swl::window_height, SDL_WINDOW_RESIZABLE);
    if(!swl_private::window)
        swl::popupError("Window could not be created!");

    swl_private::renderer = SDL_CreateRenderer(swl_private::window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if(!swl_private::renderer)
        swl::popupError("Renderer could not be created!");
    
    SDL_SetRenderDrawBlendMode(swl_private::renderer, SDL_BLENDMODE_BLEND);
    SDL_DisplayMode dm = {SDL_PIXELFORMAT_UNKNOWN, 0, 0, 0, nullptr};
    SDL_SetWindowDisplayMode(swl_private::window, &dm);

    swl_private::setResourcePath(argv[0]); // get path of resources folder
    
    int result = swl_main();

    swl::quit();
    return result;
}

void swl::popupError(const std::string& message) {
    quit();
    const SDL_MessageBoxButtonData buttons[] = {
        {SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT, 0, "close"},
    };
    const SDL_MessageBoxColorScheme colorScheme = {
        {
            /* [SDL_MESSAGEBOX_COLOR_BACKGROUND] */
            { 255,   0,   0 },
            /* [SDL_MESSAGEBOX_COLOR_TEXT] */
            {   0, 255,   0 },
            /* [SDL_MESSAGEBOX_COLOR_BUTTON_BORDER] */
            { 255, 255,   0 },
            /* [SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND] */
            {   0,   0, 255 },
            /* [SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED] */
            { 255,   0, 255 }
        }
    };
    const SDL_MessageBoxData messageboxdata = {
        SDL_MESSAGEBOX_INFORMATION, /* .flags */
        nullptr, /* .window */
        "Terralistic encountered an critical error!", /* .title */
        message.c_str(), /* .message */
        SDL_arraysize(buttons), /* .numbuttons */
        buttons, /* .buttons */
        &colorScheme /* .colorScheme */
    };
    SDL_ShowMessageBox(&messageboxdata, nullptr);
    exit(1);
}

void swl::update() {
    SDL_RenderPresent(swl_private::renderer);
}

void swl::clear() {
    SDL_RenderClear(swl_private::renderer);
}

bool swl::handleBasicEvents(SDL_Event &event, bool *running) {
    switch (event.type) {
        case SDL_QUIT:
            *running = false;
            return true;
        case SDL_WINDOWEVENT:
            if (event.window.event == SDL_WINDOWEVENT_RESIZED) {
                window_width = (unsigned short)event.window.data1;
                window_height = (unsigned short)event.window.data2;
                return true;
            }
            else
                return false;
        case SDL_MOUSEMOTION:
            int x, y;
            SDL_GetMouseState(&x, &y);
            mouse_x = (unsigned short)x;
            mouse_y = (unsigned short)y;
            return true;
        default:
            return false;
    }
}

bool swl::colliding(SDL_Rect a, SDL_Rect b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

void swl::setWindowMinimumSize(unsigned short width, unsigned short height) {
    SDL_SetWindowMinimumSize(swl_private::window, width, height);
}

void swl::setRenderTarget(SDL_Texture* texture) {
     SDL_SetRenderTarget(swl_private::renderer, texture);
}

void swl::resetRenderTarget() {
    setRenderTarget(nullptr);
}

SDL_Texture* swl::createBlankTexture(unsigned short width, unsigned short height) {
    SDL_Texture* result = SDL_CreateTexture(swl_private::renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, width, height);
    if(!result)
        swl::popupError("Blank texture could not be created!");
    return result;
}
