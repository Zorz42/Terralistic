//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//
#include "movementHandler.hpp"
#include "blockEngine.hpp"
#include "frameLengthMeasurer.hpp"

bool movementHandler::handleMovement(SDL_Event& event) {
    static bool key_up = false, key_down = false, key_left = false, key_right = false;
    if(event.type == SDL_KEYDOWN)
        switch (event.key.keysym.sym) {
            case SDLK_UP:
                if(!key_up) {
                    key_up = true;
                    velocity_y -= 1;
                }
                break;
            case SDLK_DOWN:
                if(!key_down) {
                    key_down = true;
                    velocity_y += 1;
                }
                break;
            case SDLK_LEFT:
                if(!key_left) {
                    key_left = true;
                    velocity_x -= 1;
                }
                break;
            case SDLK_RIGHT:
                if(!key_right) {
                    key_right = true;
                    velocity_x += 1;
                }
                break;
        }
    else if(event.type == SDL_KEYUP)
        switch (event.key.keysym.sym) {
            case SDLK_UP:
                key_up = false;
                velocity_y += 1;
                break;
            case SDLK_DOWN:
                key_down = false;
                velocity_y -= 1;
                break;
            case SDLK_LEFT:
                key_left = false;
                velocity_x += 1;
                break;
            case SDLK_RIGHT:
                key_right = false;
                velocity_x -= 1;
                break;
        }
    else
        return false;
    return true;
}

void movementHandler::move() {
    block_engine::position_x += velocity_x * frameLengthMeasurer::frame_length;
    block_engine::position_y += velocity_y * frameLengthMeasurer::frame_length;
}
