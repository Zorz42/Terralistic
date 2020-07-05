//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//
#include "playerHandler.hpp"
#include "blockEngine.hpp"
#include "frameLengthMeasurer.hpp"
#include "objectedGraphicsLibrary.hpp"

ogl::rect player_rect(ogl::centered);

void playerHandler::init() {
    player_rect.setWidth(BLOCK_WIDTH * 2 - 5);
    player_rect.setHeight(BLOCK_WIDTH * 3 - 5);
    player_rect.setColor(0, 0, 0);
    player_rect.fill = false;
}

bool playerHandler::handleMovement(SDL_Event& event) {
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

void playerHandler::move() {
    blockEngine::position_x += velocity_x * frameLengthMeasurer::frame_length;
    blockEngine::position_y += velocity_y * frameLengthMeasurer::frame_length;
}

void playerHandler::render() {
    player_rect.render();
}
