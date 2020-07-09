//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "framerateRegulator.hpp"
#include "SDL2/SDL.h"

void framerateRegulator::regulateFramerate() {
    static int a = 0, b = 0;
    a = SDL_GetTicks();
    frame_length = a - b;
    do {
        a = SDL_GetTicks();
        frame_length = a - b;
    } while (frame_length <= 1000/60.0);
    b = a;
}
