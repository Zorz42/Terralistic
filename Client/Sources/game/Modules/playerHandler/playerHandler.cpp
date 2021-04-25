
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "playerHandler.hpp"

// this handles player and its movement

bool isPlayerColliding();

void playerHandler::module::init() {
    player->player.setTexture(gfx::loadImageFile("texturePack/misc/player.png"));
    player->player.scale = 2;
    player->player.orientation = gfx::center;
    
    if(!multiplayer) {
        player->position_x = map->getSpawnX();
        player->position_y = map->getSpawnY() - player->player.getHeight() / 2;
    }
    map->view_x = player->position_x;
    map->view_y = player->position_y;
    
    listening_to = {packets::SPAWN_POS};
}

#define VELOCITY 20
#define JUMP_VELOCITY 80

void playerHandler::module::onKeyDown(gfx::key key) {
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
                player->player.flipped = true;
            }
            break;
        case gfx::KEY_D:
            if(!key_right) {
                key_right = true;
                player->velocity_x += VELOCITY;
                player->player.flipped = false;
            }
            break;
        case gfx::KEY_E:
            player_inventory->open = !player_inventory->open;
            if(!player_inventory->open && player_inventory->getMouseItem()->item_id != map::itemType::NOTHING) {
                unsigned char result = player_inventory->addItem(player_inventory->getMouseItem()->item_id, player_inventory->getMouseItem()->getStack());
                player_inventory->clearMouseItem();
                packets::packet packet(packets::INVENTORY_SWAP);
                packet << result;
                manager->sendPacket(packet);
            }
            break;
        case gfx::KEY_MOUSE_LEFT: {
            if(hovered) {
                player_inventory->swapWithMouseItem(hovered);
                if(multiplayer) {
                    packets::packet packet(packets::INVENTORY_SWAP);
                    packet << (unsigned char)(hovered - &player_inventory->inventory[0]);
                    manager->sendPacket(packet);
                }
            }
            break;
        }
        default:;
    }
}

void playerHandler::module::onKeyUp(gfx::key key) {
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
            key_left = false;
            player->velocity_x += VELOCITY;
            break;
        case gfx::KEY_D:
            key_right = false;
            player->velocity_x -= VELOCITY;
            break;
        default:;
    }
}

bool playerHandler::module::isPlayerColliding() {
    if(player->position_x < player->player.getWidth() / 2 || player->position_y < player->player.getHeight() / 2 ||
       player->position_y >= map->getWorldHeight() * BLOCK_WIDTH - player->player.getHeight() / 2 ||
       player->position_x >= map->getWorldWidth() * BLOCK_WIDTH - player->player.getWidth() / 2)
        return true;

    unsigned short starting_x = (player->position_x - player->player.getWidth() / 2) / BLOCK_WIDTH;
    unsigned short starting_y = (player->position_y - player->player.getHeight() / 2) / BLOCK_WIDTH;
    unsigned short ending_x = (player->position_x + player->player.getWidth() / 2 - 1) / BLOCK_WIDTH;
    unsigned short ending_y = (player->position_y + player->player.getHeight() / 2 - 1) / BLOCK_WIDTH;
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            if(map->getChunkState(x >> 4, y >> 4) != map::chunkState::loaded ||
               !map->getBlock(x, y).isGhost())
                return true;
    
    return false;
}

bool playerHandler::module::touchingGround() {
    player->position_y++;
    bool result = isPlayerColliding();
    player->position_y--;
    return result;
}

void playerHandler::module::update() {
    // gravity
    player->velocity_y = touchingGround() && player->velocity_y >= 0 ? short(0) : short(player->velocity_y + gfx::getDeltaTime() / 4);
    
    int move_x = player->velocity_x * gfx::getDeltaTime() / 100, move_y = player->velocity_y * gfx::getDeltaTime() / 100;
    
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
    
    map->view_x = player->position_x;
    map->view_y = player->position_y;
    if(map->view_x < gfx::getWindowWidth() / 2)
        map->view_x = gfx::getWindowWidth() / 2;
    if(map->view_y < gfx::getWindowHeight() / 2)
        map->view_y = gfx::getWindowHeight() / 2;
    if(map->view_x >= map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
        map->view_x = map->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
    if(map->view_y >= map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
        map->view_y = map->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
    
    if(multiplayer && (move_x || move_y)) {
        packets::packet packet(packets::PLAYER_MOVEMENT);
        packet << player->position_x << player->position_y << (char)player->player.flipped;
        manager->sendPacket(packet);
    }
    
    // look for items to be picked up
    if(!multiplayer)
        for(unsigned long i = 0; i < map->items.size(); i++)
            if(abs(map->items[i].x / 100 + BLOCK_WIDTH / 2  - player->position_x - player->player.getWidth() / 2) < 50 && abs(map->items[i].y / 100 + BLOCK_WIDTH / 2 - player->position_y - player->player.getHeight() / 2) < 50 && player_inventory->addItem(map->items[i].getItemId(), 1) != -1) {
                map->items[i].destroy();
                map->items.erase(map->items.begin() + i);
            }
}

void playerHandler::module::render() {
    player->player.x = player->position_x - map->view_x;
    player->player.y = player->position_y - map->view_y;
    gfx::render(player->player);
}

void playerHandler::module::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::SPAWN_POS: {
            int x = packet.getInt(), y = packet.getInt();
            player->position_x = x;
            player->position_y = y;
            map->view_x = x;
            map->view_y = y;
            break;
        }
        default:;
    }
}
