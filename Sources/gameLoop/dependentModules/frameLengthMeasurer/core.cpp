//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "frameLengthMeasurer.hpp"
#include "SDL2/SDL.h"

void frameLengthMeasurer::measureFrameLength() {
    static unsigned int ticks = SDL_GetTicks();
    frame_length = SDL_GetTicks() - ticks;
    ticks = SDL_GetTicks();
}
