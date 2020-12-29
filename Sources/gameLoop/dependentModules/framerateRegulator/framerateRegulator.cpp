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
    frame_length = static_cast<unsigned short>(a - b);
    if(frame_length < 1000 / fps_limit)
        SDL_Delay(static_cast<Uint32>(1000 / fps_limit - frame_length));
    b = a;
}
