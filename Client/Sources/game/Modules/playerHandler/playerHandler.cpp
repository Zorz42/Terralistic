
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "game.hpp"
#include "itemRenderer.hpp"
#include "blockRenderer.hpp"

#define INC_X position_x++; playerHandler::view_x++
#define DEC_X position_x--; playerHandler::view_x--
#define INC_Y position_y++; playerHandler::view_y++
#define DEC_Y position_y--; playerHandler::view_y--

// this handles player and its movement

bool isPlayerColliding();

void playerHandler::module::init() {
    playerHandler::player.setTexture(gfx::loadImageFile("texturePack/misc/player.png"));
    playerHandler::player.scale = 2;
    playerHandler::player.orientation = gfx::center;
    
    if(!scene->multiplayer) {
        position_x = scene->world_map.getSpawnX();
        position_y = scene->world_map.getSpawnY() - player.getHeight() / 2;
    }
    view_x = position_x;
    view_y = position_y;
    
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
                velocity_x -= VELOCITY;
                player.flipped = true;
            }
            break;
        case gfx::KEY_D:
            if(!key_right) {
                key_right = true;
                velocity_x += VELOCITY;
                player.flipped = false;
            }
            break;
        case gfx::KEY_E:
            scene->player_inventory.open = !scene->player_inventory.open;
            if(!scene->player_inventory.open && scene->player_inventory.getMouseItem()->item_id != itemEngine::NOTHING) {
                unsigned char result = scene->player_inventory.addItem(scene->player_inventory.getMouseItem()->item_id, scene->player_inventory.getMouseItem()->getStack());
                scene->player_inventory.clearMouseItem();
                packets::packet packet(packets::INVENTORY_SWAP);
                packet << result;
                scene->networking_manager.sendPacket(packet);
            }
            break;
        case gfx::KEY_MOUSE_LEFT: {
            if(hovered) {
                scene->player_inventory.swapWithMouseItem(hovered);
                if(scene->multiplayer) {
                    packets::packet packet(packets::INVENTORY_SWAP);
                    packet << (unsigned char)(hovered - &scene->player_inventory.inventory[0]);
                    scene->networking_manager.sendPacket(packet);
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
                if(velocity_y < -10)
                    velocity_y = -10;
            }
            break;
        case gfx::KEY_A:
            key_left = false;
            velocity_x += VELOCITY;
            break;
        case gfx::KEY_D:
            key_right = false;
            velocity_x -= VELOCITY;
            break;
        default:;
    }
}

bool playerHandler::module::isPlayerColliding() {
#define COLLISION_PADDING 2
    
    if(position_x < playerHandler::player.getWidth() / 2 || position_y < playerHandler::player.getHeight() / 2 ||
       position_y >= scene->world_map.getWorldHeight() * BLOCK_WIDTH - playerHandler::player.getHeight() / 2 ||
       position_x >= scene->world_map.getWorldWidth() * BLOCK_WIDTH - playerHandler::player.getWidth() / 2)
        return true;

    short begin_x = position_x / BLOCK_WIDTH - playerHandler::player.getWidth() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    short end_x = position_x / BLOCK_WIDTH + playerHandler::player.getWidth() / 2 / BLOCK_WIDTH + COLLISION_PADDING;

    short begin_y = position_y / BLOCK_WIDTH - playerHandler::player.getHeight() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    short end_y = position_y / BLOCK_WIDTH + playerHandler::player.getHeight() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > scene->world_map.getWorldWidth())
        end_x = scene->world_map.getWorldWidth();
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > scene->world_map.getWorldHeight())
        end_y = scene->world_map.getWorldHeight();
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(scene->world_map.getChunkState(x >> 4, y >> 4) != scene->world_map.loaded || (gfx::colliding(gfx::rectShape(short(x * BLOCK_WIDTH - playerHandler::view_x + gfx::getWindowWidth() / 2), short(y * BLOCK_WIDTH - playerHandler::view_y + gfx::getWindowHeight() / 2), BLOCK_WIDTH, BLOCK_WIDTH), playerHandler::player.getTranslatedRect()) && !scene->world_map.getBlock(x, y).getUniqueBlock().ghost))
                return true;
    return false;
}

bool playerHandler::module::touchingGround() {
    INC_Y;
    bool result = isPlayerColliding();
    DEC_Y;
    return result;
}

void playerHandler::module::update() {
    // gravity
    velocity_y = touchingGround() && velocity_y >= 0 ? short(0) : short(velocity_y + gfx::getDeltaTime() / 4);
    
    int move_x = velocity_x * gfx::getDeltaTime() / 100, move_y = velocity_y * gfx::getDeltaTime() / 100;
    
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
    
    view_x = position_x;
    view_y = position_y;
    if(view_x < gfx::getWindowWidth() / 2)
        view_x = gfx::getWindowWidth() / 2;
    if(view_y < gfx::getWindowHeight() / 2)
        view_y = gfx::getWindowHeight() / 2;
    if(view_x >= scene->world_map.getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
        view_x = scene->world_map.getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
    if(view_y >= scene->world_map.getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
        view_y = scene->world_map.getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
    
    if(scene->multiplayer && (move_x || move_y)) {
        packets::packet packet(packets::PLAYER_MOVEMENT);
        packet << position_x << position_y << (char)player.flipped;
        scene->networking_manager.sendPacket(packet);
    }
    
    // look for items to be picked up
    if(!scene->multiplayer)
        for(unsigned long i = 0; i < itemEngine::items.size(); i++)
            if(abs(itemEngine::items[i].x / 100 + BLOCK_WIDTH / 2  - position_x - playerHandler::player.getWidth() / 2) < 50 && abs(itemEngine::items[i].y / 100 + BLOCK_WIDTH / 2 - position_y - playerHandler::player.getHeight() / 2) < 50 && scene->player_inventory.addItem(itemEngine::items[i].getItemId(), 1) != -1) {
                itemEngine::items[i].destroy();
                itemEngine::items.erase(itemEngine::items.begin() + i);
            }
}

void playerHandler::module::render() {
    player.x = position_x - view_x;
    player.y = position_y - view_y;
    gfx::render(player);
}

void playerHandler::module::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::SPAWN_POS: {
            int x = packet.getInt(), y = packet.getInt();
            position_x = x;
            position_y = y;
            playerHandler::view_x = x;
            playerHandler::view_y = y;
            break;
        }
        default:;
    }
}
