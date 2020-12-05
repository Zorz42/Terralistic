//
//  physics.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "playerHandler.hpp"
#include "blockEngine.hpp"

void playerHandler::doPhysics() {
    if(touchingGround() && velocity_y >= 0)
        velocity_y = 0;
    else
        velocity_y += 4;
}

bool playerHandler::touchingGround() {
#define INC_X blockEngine::position_x++;blockEngine::view_x++
#define DEC_X blockEngine::position_x--;blockEngine::view_x--
#define INC_Y blockEngine::position_y++;blockEngine::view_y++
#define DEC_Y blockEngine::position_y--;blockEngine::view_y--
    INC_Y;
    bool result = isPlayerColliding();
    DEC_Y;
    return result;
}
