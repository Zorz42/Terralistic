//
//  movementHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef movementHandler_h
#define movementHandler_h

#include "SDL2/SDL.h"

namespace movementHandler {

bool handleMovement(SDL_Event& event);
void move();

inline int velocity_x = 0, velocity_y = 0;

}

#endif /* movementHandler_h */
