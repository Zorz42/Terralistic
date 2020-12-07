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
    if(frame_length < 16)
        SDL_Delay(16 - frame_length);
    b = a;
}
