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
#include "networkingModule.hpp"
#include "gameLoop.hpp"

bool key_up = false, jump = false;

void playerHandler::init() {
    player.loadFromFile("texturePack/player.png");
    player.scale = 2;
}

bool playerHandler::handleMovement(SDL_Event& event) {
#define VELOCITY 20
#define JUMP_VELOCITY 80
    static bool key_left = false, key_right = false;
    if(event.type == SDL_KEYDOWN)
        switch (event.key.keysym.sym) {
            case SDLK_SPACE:
                if(!key_up) {
                    key_up = true;
                    jump = true;
                }
                break;
            case SDLK_a:
                if(!key_left) {
                    key_left = true;
                    velocity_x -= VELOCITY;
                    player.flipped = true;
                }
                break;
            case SDLK_d:
                if(!key_right) {
                    key_right = true;
                    velocity_x += VELOCITY;
                    player.flipped = false;
                }
                break;
            default:
                return false;
        }
    else if(event.type == SDL_KEYUP)
        switch (event.key.keysym.sym) {
            case SDLK_SPACE:
                if(key_up) {
                    key_up = false;
                    jump = false;
                    if(velocity_y < -10)
                        velocity_y = -10;
                }
                break;
            case SDLK_a:
                key_left = false;
                velocity_x += VELOCITY;
                break;
            case SDLK_d:
                key_right = false;
                velocity_x -= VELOCITY;
                break;
            default:
                return false;
        }
    else
        return false;
    return true;
}

bool playerHandler::isPlayerColliding() {
#define COLLISION_PADDING 2
    
    if(blockEngine::position_x < playerHandler::player.getWidth() / 2 || blockEngine::position_y < playerHandler::player.getHeight() / 2 ||
       blockEngine::position_y >= blockEngine::world_height * BLOCK_WIDTH - playerHandler::player.getHeight() / 2 ||
       blockEngine::position_x >= blockEngine::world_width * BLOCK_WIDTH - playerHandler::player.getWidth() / 2)
        return true;
    
    int begin_x = blockEngine::position_x / BLOCK_WIDTH - playerHandler::player.getWidth() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    int end_x = blockEngine::position_x / BLOCK_WIDTH + playerHandler::player.getWidth() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    int begin_y = blockEngine::position_y / BLOCK_WIDTH - playerHandler::player.getHeight() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    int end_y = blockEngine::position_y / BLOCK_WIDTH + playerHandler::player.getHeight() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > blockEngine::world_width)
        end_x = blockEngine::world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > blockEngine::world_height)
        end_y = blockEngine::world_height;
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(swl::colliding(blockEngine::getBlock(x, y).getRect(x, y), playerHandler::player.getRect()) && !blockEngine::getBlock(x, y).getUniqueBlock().ghost)
                return true;
    return false;
}

void playerHandler::move() {
#define INC_X blockEngine::position_x++;blockEngine::view_x++
#define DEC_X blockEngine::position_x--;blockEngine::view_x--
#define INC_Y blockEngine::position_y++;blockEngine::view_y++
#define DEC_Y blockEngine::position_y--;blockEngine::view_y--
    int move_x = velocity_x * framerateRegulator::frame_length / 100, move_y = velocity_y * framerateRegulator::frame_length / 100;
    for(int i = 0; i < move_x; i++) {
        INC_X;
        if(isPlayerColliding()) {
            DEC_X;
            break;
        }
    }
    for(int i = 0; i > move_x; i--) {
        DEC_X;
        if(isPlayerColliding()) {
            INC_X;
            break;
        }
    }
    for(int i = 0; i < move_y; i++) {
        INC_Y;
        if(isPlayerColliding()) {
            DEC_Y;
            break;
        }
    }
    for(int i = 0; i > move_y; i--) {
        DEC_Y;
        if(isPlayerColliding()) {
            INC_Y;
            if(velocity_y < 0)
                velocity_y = 0;
            break;
        }
    }
    if(touchingGround() && jump) {
        velocity_y = -JUMP_VELOCITY;
        jump = false;
    }
    blockEngine::view_x = blockEngine::position_x;
    blockEngine::view_y = blockEngine::position_y;
    
    if(gameLoop::online && (move_x || move_y)) {
        packets::packet packet(packets::PLAYER_MOVEMENT);
        packet << blockEngine::position_x << blockEngine::position_y;
        networking::sendPacket(packet);
    }
}

void playerHandler::render() {    
    player.setX(short(blockEngine::position_x - blockEngine::view_x));
    player.setY(short(blockEngine::position_y - blockEngine::view_y));
    player.render();
}

void playerHandler::doPhysics() {
    velocity_y = touchingGround() && velocity_y >= 0 ? short(0) : short(velocity_y + framerateRegulator::frame_length / 4);
}

bool playerHandler::touchingGround() {
    INC_Y;
    bool result = isPlayerColliding();
    DEC_Y;
    return result;
}
