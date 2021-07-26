
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "playerHandler.hpp"

// this handles player and its movement

void playerHandler::init() {
    initRenderer();
    
    world_map->view_x = player->x;
    world_map->view_y = player->y;
    
    sf::Packet join_packet;
    join_packet << player->name;
    manager->sendPacket(join_packet);
    
    sf::Packet packet;
    packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH);
    manager->sendPacket(packet);
}

//#define VELOCITY 30
//#define JUMP_VELOCITY 80
#define VELOCITY 90
#define JUMP_VELOCITY 240

void playerHandler::onKeyDown(gfx::Key key) {
    switch(key) {
        case gfx::Key::SPACE:
            if(!key_up) {
                key_up = true;
                jump = true;
            }
            break;
        case gfx::Key::A:
            if(!key_left) {
                key_left = true;
                player->velocity_x -= VELOCITY;
                player->flipped = true;
            }
            break;
        case gfx::Key::D:
            if(!key_right) {
                key_right = true;
                player->velocity_x += VELOCITY;
                player->flipped = false;
            }
            break;
        default:;
    }
}

void playerHandler::onKeyUp(gfx::Key key) {
    switch (key) {
        case gfx::Key::SPACE:
            if(key_up) {
                key_up = false;
                jump = false;
                if(player->velocity_y < -10)
                    player->velocity_y = -10;
            }
            break;
        case gfx::Key::A:
            if(key_left) {
                key_left = false;
                player->velocity_x += VELOCITY;
            }
            break;
        case gfx::Key::D:
            if(key_right) {
                key_right = false;
                player->velocity_x -= VELOCITY;
            }
            break;
        default:;
    }
}

bool playerHandler::isPlayerColliding() {
    if(player->x < getPlayerWidth() / 2 || player->y < getPlayerHeight() / 2 ||
       player->y >= world_map->getWorldHeight() * BLOCK_WIDTH - getPlayerHeight() / 2 ||
       player->x >= world_map->getWorldWidth() * BLOCK_WIDTH - getPlayerWidth() / 2)
        return true;

    unsigned short starting_x = (player->x - getPlayerWidth() / 2) / BLOCK_WIDTH;
    unsigned short starting_y = (player->y - getPlayerHeight() / 2) / BLOCK_WIDTH;
    unsigned short ending_x = (player->x + getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
    unsigned short ending_y = (player->y + getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            if(world_map->getChunk(x >> 4, y >> 4).getState() != map::chunkState::loaded || !world_map->getBlock(x, y).isGhost())
                return true;
    
    return false;
}

bool playerHandler::touchingGround() {
    player->y++;
    bool result = isPlayerColliding();
    player->y--;
    return result;
}

void playerHandler::update() {
    if(received_spawn_coords) {
        static unsigned short prev_width = gfx::getWindowWidth(), prev_height = gfx::getWindowHeight();
        if(prev_width != gfx::getWindowWidth() || prev_height != gfx::getWindowHeight()) {
            sf::Packet packet;
            packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH);
            manager->sendPacket(packet);
            
            prev_width = gfx::getWindowWidth();
            prev_height = gfx::getWindowHeight();
        }
        
        float speed_multiplier = 1;
        
        int prev_x = player->x, prev_y = player->y, prev_view_x = world_map->view_x, prev_view_y = world_map->view_y;
        
        unsigned short starting_x = (player->x - getPlayerWidth() / 2) / BLOCK_WIDTH;
        unsigned short starting_y = (player->y - getPlayerHeight() / 2) / BLOCK_WIDTH;
        unsigned short ending_x = (player->x + getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
        unsigned short ending_y = (player->y + getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
        
        for(unsigned short x = starting_x; x <= ending_x; x++)
            for(unsigned short y = starting_y; y <= ending_y; y++)
                speed_multiplier = std::min(speed_multiplier, world_map->getBlock(x, y).getSpeedMultiplier());
        
        player->velocity_y = touchingGround() && player->velocity_y >= 0 ? short(0) : short(player->velocity_y + gfx::getDeltaTime() / 4 * speed_multiplier);
        
        int move_x = float(player->velocity_x * gfx::getDeltaTime() / 100) * speed_multiplier, move_y = float(player->velocity_y * gfx::getDeltaTime() / 100) * speed_multiplier;
        
        for(int i = 0; i < move_x; i++) {
            player->x++;
            if(isPlayerColliding()) {
                player->x--;
                break;
            }
        }
        for(int i = 0; i > move_x; i--) {
            player->x--;
            if(isPlayerColliding()) {
                player->x++;
                break;
            }
        }
        for(int i = 0; i < move_y; i++) {
            player->y++;
            if(isPlayerColliding()) {
                player->y--;
                break;
            }
        }
        for(int i = 0; i > move_y; i--) {
            player->y--;
            if(isPlayerColliding()) {
                player->y++;
                if(player->velocity_y < 0)
                    player->velocity_y = 0;
                break;
            }
        }
        if(touchingGround() && jump) {
            player->velocity_y = -JUMP_VELOCITY;
            jump = false;
        }
        
        world_map->view_x += (player->x - world_map->view_x) / 8;
        world_map->view_y += (player->y - world_map->view_y) / 8;
        if(world_map->view_x < gfx::getWindowWidth() / 2)
            world_map->view_x = gfx::getWindowWidth() / 2;
        if(world_map->view_y < gfx::getWindowHeight() / 2)
            world_map->view_y = gfx::getWindowHeight() / 2;
        if(world_map->view_x >= world_map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
            world_map->view_x = world_map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
        if(world_map->view_y >= world_map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
            world_map->view_y = world_map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
        
        if(prev_x != player->x || prev_y != player->y) {
            sf::Packet packet;
            packet << PacketType::PLAYER_MOVEMENT << player->x << player->y << player->flipped;
            manager->sendPacket(packet);
        }
        
        if(prev_view_x != world_map->view_x || prev_view_y != world_map->view_y) {
            sf::Packet packet;
            packet << PacketType::VIEW_POS_CHANGE << world_map->view_x << world_map->view_y;
            manager->sendPacket(packet);
        }
    }
}
