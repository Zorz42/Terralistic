
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "playerHandler.hpp"
#include "playerRenderer.hpp"

// this handles player and its movement

bool isPlayerColliding();

void playerHandler::init() {
    playerRenderer::init();
    
    world_map->view_x = player->position_x;
    world_map->view_y = player->position_y;
    
    manager->sendPacket(packets::PLAYER_JOIN);
    initInventory();
}

#define VELOCITY 20
#define JUMP_VELOCITY 80

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
        case gfx::KEY_E:
            player->player_inventory.open = !player->player_inventory.open;
            if(!player->player_inventory.open && player->player_inventory.getMouseItem()->item_id != map::itemType::NOTHING) {
                unsigned char result = player->player_inventory.addItem(player->player_inventory.getMouseItem()->item_id, player->player_inventory.getMouseItem()->getStack());
                player->player_inventory.clearMouseItem();
                packets::packet packet(packets::INVENTORY_SWAP);
                packet << result;
                manager->sendPacket(packet);
            }
            break;
        case gfx::KEY_MOUSE_LEFT: {
            if(hovered) {
                player->player_inventory.swapWithMouseItem(hovered);
                packets::packet packet(packets::INVENTORY_SWAP);
                packet << (unsigned char)(hovered - &player->player_inventory.inventory[0]);
                manager->sendPacket(packet);
            }
            break;
        }
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
            key_left = false;
            player->velocity_x += VELOCITY;
            break;
        case gfx::KEY_D:
            key_right = false;
            player->velocity_x -= VELOCITY;
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
            if(world_map->getChunk(x >> 4, y >> 4).getState() != map::chunkState::loaded ||
               !world_map->getBlock(x, y).isGhost())
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
            break;
        }
        default:;
    }
    onPacketInventory(packet);
}
