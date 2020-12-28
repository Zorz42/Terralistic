//
//  swl.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#include <SDL2_image/SDL_image.h>
#include "singleWindowLibrary.hpp"

void swl::quit() {
    SDL_DestroyRenderer(__swl_private::renderer);
    __swl_private::renderer = nullptr;
    SDL_DestroyWindow(__swl_private::window);
    __swl_private::window = nullptr;
    SDL_Quit();
}

//#undef main

int main(int argc, char **argv) {
    swl::window_width = 1000;
    swl::window_height = 600;
    
    if(SDL_Init(SDL_INIT_EVERYTHING) < 0)
        swl::popupError("SDL could not initialize properly!");

    int img_flags = IMG_INIT_PNG;
    if(!(IMG_Init(img_flags) & img_flags))
        swl::popupError("SDL_image could not initialize properly!");
    
    if(TTF_Init() == -1)
        swl::popupError("SDL_ttf could not initialize properly!");
    
    __swl_private::window = SDL_CreateWindow("Terralistic", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, swl::window_width, swl::window_height, SDL_WINDOW_RESIZABLE);
    if(!__swl_private::window)
        swl::popupError("Window could not be created!");

    __swl_private::renderer = SDL_CreateRenderer(__swl_private::window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if(!__swl_private::renderer)
        swl::popupError("Renderer could not be created!");
    
    SDL_SetRenderDrawBlendMode(__swl_private::renderer, SDL_BLENDMODE_BLEND);
    SDL_DisplayMode dm = {SDL_PIXELFORMAT_UNKNOWN, 0, 0, 0, 0};
    SDL_SetWindowDisplayMode(__swl_private::window, &dm);

    __swl_private::setResourcePath(argv[0]); // get path of resources folder
    
    int result = swl_main();

    swl::quit();
    return result;
}

void swl::popupError(std::string message) {
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
        NULL, /* .window */
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
    SDL_RenderPresent(__swl_private::renderer);
}

void swl::clear() {
    SDL_RenderClear(__swl_private::renderer);
}

bool swl::handleBasicEvents(SDL_Event &event, bool *running) {
    if(event.type == SDL_QUIT) {
        *running = false;
        return true;
    }
    else if(event.type == SDL_WINDOWEVENT) {
        if (event.window.event == SDL_WINDOWEVENT_RESIZED) {
            window_width = event.window.data1;
            window_height = event.window.data2;
            return true;
        }
        else
            return false;
    }
    else if(event.type == SDL_MOUSEMOTION) {
        SDL_GetMouseState(&mouse_x, &mouse_y);
        return true;
    }
    return false;
}

bool swl::colliding(SDL_Rect a, SDL_Rect b) {
    //The sides of the rectangles
    int leftA, leftB;
    int rightA, rightB;
    int topA, topB;
    int bottomA, bottomB;

    //Calculate the sides of rect A
    leftA = a.x;
    rightA = a.x + a.w;
    topA = a.y;
    bottomA = a.y + a.h;

    //Calculate the sides of rect B
    leftB = b.x;
    rightB = b.x + b.w;
    topB = b.y;
    bottomB = b.y + b.h;
    //If any of the sides from A are outside of B
    if(bottomA <= topB)
        return false;

    if(topA >= bottomB)
        return false;

    if(rightA <= leftB)
        return false;

    if(leftA >= rightB)
        return false;

    //If none of the sides from A are outside B
    return true;
}

void swl::setWindowMinimumSize(int width, int height) {
    SDL_SetWindowMinimumSize(__swl_private::window, width, height);
}
