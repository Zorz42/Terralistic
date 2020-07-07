//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//
#include "playerHandler.hpp"
#include "blockEngine.hpp"
#include "framerateRegulator.hpp"
#include "singleWindowLibrary.hpp"
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

bool isPlayerColliding() {
#define COLLISION_PADDING 2

    if(blockEngine::position_x < player_rect.getWidth() / 2 || blockEngine::position_y < player_rect.getHeight() / 2 ||
       blockEngine::position_y >= blockEngine::world_height * BLOCK_WIDTH - player_rect.getHeight() / 2 ||
       blockEngine::position_x >= blockEngine::world_width * BLOCK_WIDTH - player_rect.getWidth() / 2)
        return true;
    
    int begin_x = (int)blockEngine::position_x / BLOCK_WIDTH - player_rect.getWidth() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    int end_x = (int)blockEngine::position_x / BLOCK_WIDTH + player_rect.getWidth() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    int begin_y = (int)blockEngine::position_y / BLOCK_WIDTH - player_rect.getHeight() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    int end_y = (int)blockEngine::position_y / BLOCK_WIDTH + player_rect.getHeight() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > blockEngine::world_width)
        end_x = blockEngine::world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > blockEngine::world_height)
        end_y = blockEngine::world_height;
    
    for(int x = begin_x; x < end_x; x++)
        for(int y = begin_y; y < end_y; y++)
            if(swl::colliding(blockEngine::getBlock(x, y).getRect(), player_rect.getRect()) && blockEngine::getBlock(x, y).block_id != blockEngine::BLOCK_AIR)
                return true;
    return false;
}

void playerHandler::move() {
#define INC_X blockEngine::position_x++;blockEngine::view_x++
#define DEC_X blockEngine::position_x--;blockEngine::view_x--
#define INC_Y blockEngine::position_y++;blockEngine::view_y++
#define DEC_Y blockEngine::position_y--;blockEngine::view_y--
    for(int i = 0; i < velocity_x * framerateRegulator::frame_length; i++) {
        INC_X;
        if(isPlayerColliding()) {
            DEC_X;
            break;
        }
    }
    for(int i = 0; i > velocity_x * framerateRegulator::frame_length; i--) {
        DEC_X;
        if(isPlayerColliding()) {
            INC_X;
            break;
        }
    }
    for(int i = 0; i < velocity_y * framerateRegulator::frame_length; i++) {
        INC_Y;
        if(isPlayerColliding()) {
            DEC_Y;
            break;
        }
    }
    for(int i = 0; i > velocity_y * framerateRegulator::frame_length; i--) {
        DEC_Y;
        if(isPlayerColliding()) {
            INC_Y;
            break;
        }
    }
    blockEngine::view_x = blockEngine::position_x;
    blockEngine::view_y = blockEngine::position_y;
}

void playerHandler::render() {
    player_rect.setX(blockEngine::position_x - blockEngine::view_x);
    player_rect.setY(blockEngine::position_y - blockEngine::view_y);
    player_rect.render();
}
