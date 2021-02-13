//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "framerateRegulator.hpp"
#include "SDL2/SDL.h"

void framerateRegulator::regulateFramerate() {
    // just regulate framerate, wait that much time, that the frame if fps_limit or lower. never higher
    static int a = 0, b = SDL_GetTicks();
    a = SDL_GetTicks();
    frame_length = (unsigned short)(a - b);
    if(frame_length < 1000 / fps_limit)
        SDL_Delay(Uint32(1000 / fps_limit - frame_length));
    b = a;
}
