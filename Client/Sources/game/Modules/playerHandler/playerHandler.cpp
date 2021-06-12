
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "playerHandler.hpp"
#include "playerRenderer.hpp"

// this handles player and its movement

void playerHandler::init() {
    playerRenderer::init();
    
    world_map->view_x = player->position_x;
    world_map->view_y = player->position_y;
    
    packets::packet join_packet(packets::PLAYER_JOIN);
    join_packet << player->name;
    manager->sendPacket(join_packet);
    
    packets::packet packet(packets::VIEW_SIZE_CHANGE);
    packet << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH);
    manager->sendPacket(packet);
    
    initInventory();
}

#define VELOCITY 60
#define JUMP_VELOCITY 800

void playerHandler::onKeyDown(gfx::key key) {
    switch(key) {
        case gfx::KEY_SPACE:
            if(!key_up) {
                key_up = true;
                jump = true;
            }
            break;
        case gfx::KEY_A:
            if(!key_left) {
                key_left = true;
                player->velocity_x -= VELOCITY;
                player->flipped = true;
            }
            break;
        case gfx::KEY_D:
            if(!key_right) {
                key_right = true;
                player->velocity_x += VELOCITY;
                player->flipped = false;
            }
            break;
        default:;
    }
    
    onKeyDownInventory(key);
    onKeyDownSelector(key);
}

void playerHandler::onKeyUp(gfx::key key) {
    switch (key) {
        case gfx::KEY_SPACE:
            if(key_up) {
                key_up = false;
                jump = false;
                if(player->velocity_y < -10)
                    player->velocity_y = -10;
            }
            break;
        case gfx::KEY_A:
            if(key_left) {
                key_left = false;
                player->velocity_x += VELOCITY;
            }
            break;
        case gfx::KEY_D:
            if(key_right) {
                key_right = false;
                player->velocity_x -= VELOCITY;
            }
            break;
        default:;
    }
    onKeyUpSelector(key);
}

bool playerHandler::isPlayerColliding() {
    if(player->position_x < playerRenderer::getPlayerWidth() / 2 || player->position_y < playerRenderer::getPlayerHeight() / 2 ||
       player->position_y >= world_map->getWorldHeight() * BLOCK_WIDTH - playerRenderer::getPlayerHeight() / 2 ||
       player->position_x >= world_map->getWorldWidth() * BLOCK_WIDTH - playerRenderer::getPlayerWidth() / 2)
        return true;

    unsigned short starting_x = (player->position_x - playerRenderer::getPlayerWidth() / 2) / BLOCK_WIDTH;
    unsigned short starting_y = (player->position_y - playerRenderer::getPlayerHeight() / 2) / BLOCK_WIDTH;
    unsigned short ending_x = (player->position_x + playerRenderer::getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
    unsigned short ending_y = (player->position_y + playerRenderer::getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            if(world_map->getChunk(x >> 4, y >> 4).getState() != map::chunkState::loaded || !world_map->getBlock(x, y).isGhost())
                return true;
    
    return false;
}

bool playerHandler::touchingGround() {
    player->position_y++;
    bool result = isPlayerColliding();
    player->position_y--;
    return result;
}

void playerHandler::update() {
    if(received_spawn_coords) {
        static unsigned short prev_width = gfx::getWindowWidth(), prev_height = gfx::getWindowHeight();
        if(prev_width != gfx::getWindowWidth() || prev_height != gfx::getWindowHeight()) {
            packets::packet packet(packets::VIEW_SIZE_CHANGE);
            packet << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH);
            manager->sendPacket(packet);
            
            prev_width = gfx::getWindowWidth();
            prev_height = gfx::getWindowHeight();
        }
        
        float speed_multiplier = 1;
        
        unsigned short starting_x = (player->position_x - playerRenderer::getPlayerWidth() / 2) / BLOCK_WIDTH;
        unsigned short starting_y = (player->position_y - playerRenderer::getPlayerHeight() / 2) / BLOCK_WIDTH;
        unsigned short ending_x = (player->position_x + playerRenderer::getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
        unsigned short ending_y = (player->position_y + playerRenderer::getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
        
        for(unsigned short x = starting_x; x <= ending_x; x++)
            for(unsigned short y = starting_y; y <= ending_y; y++)
                speed_multiplier = std::min(speed_multiplier, world_map->getBlock(x, y).getSpeedMultiplier());
        
        // gravity
        player->velocity_y = touchingGround() && player->velocity_y >= 0 ? short(0) : short(player->velocity_y + gfx::getDeltaTime() / 4 * speed_multiplier);
        
        int move_x = float(player->velocity_x * gfx::getDeltaTime() / 100) * speed_multiplier, move_y = float(player->velocity_y * gfx::getDeltaTime() / 100) * speed_multiplier;
        
        for(int i = 0; i < move_x; i++) {
            player->position_x++;
            if(isPlayerColliding()) {
                player->position_x--;
                break;
            }
        }
        for(int i = 0; i > move_x; i--) {
            player->position_x--;
            if(isPlayerColliding()) {
                player->position_x++;
                break;
            }
        }
        for(int i = 0; i < move_y; i++) {
            player->position_y++;
            if(isPlayerColliding()) {
                player->position_y--;
                break;
            }
        }
        for(int i = 0; i > move_y; i--) {
            player->position_y--;
            if(isPlayerColliding()) {
                player->position_y++;
                if(player->velocity_y < 0)
                    player->velocity_y = 0;
                break;
            }
        }
        if(touchingGround() && jump) {
            player->velocity_y = -JUMP_VELOCITY;
            jump = false;
        }
        
        world_map->view_x = player->position_x;
        world_map->view_y = player->position_y;
        if(world_map->view_x < gfx::getWindowWidth() / 2)
            world_map->view_x = gfx::getWindowWidth() / 2;
        if(world_map->view_y < gfx::getWindowHeight() / 2)
            world_map->view_y = gfx::getWindowHeight() / 2;
        if(world_map->view_x >= world_map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
            world_map->view_x = world_map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
        if(world_map->view_y >= world_map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
            world_map->view_y = world_map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
        
        if(move_x || move_y) {
            packets::packet packet(packets::PLAYER_MOVEMENT);
            packet << player->position_x << player->position_y << (char)player->flipped;
            manager->sendPacket(packet);
        }
    }
}

void playerHandler::render() {
    playerRenderer::render(player->position_x, player->position_y, world_map->view_x, world_map->view_y, player->flipped);
    renderBlockSelector();
    renderInventory();
}

void playerHandler::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::SPAWN_POS: {
            int x = packet.getInt(), y = packet.getInt();
            player->position_x = x;
            player->position_y = y;
            world_map->view_x = x;
            world_map->view_y = y;
            received_spawn_coords = true;
            break;
        }
        default:;
    }
    onPacketInventory(packet);
}
