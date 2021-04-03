
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
#include "itemEngineClient.hpp"
#include "blockRenderer.hpp"

#define INC_X position_x++; playerHandler::view_x++
#define DEC_X position_x--; playerHandler::view_x--
#define INC_Y position_y++; playerHandler::view_y++
#define DEC_Y position_y--; playerHandler::view_y--

#define MARGIN 10

// this handles player and its movement

bool isPlayerColliding();

void playerHandler::module::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].orientation = gfx::top;
        inventory_slots[i].c = {100, 100, 100};
        inventory_slots[i].h = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].w = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].x = (i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
        inventory_slots[i].y = MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH;
    }
    
    playerHandler::player.setTexture(gfx::loadImageFile("texturePack/misc/player.png"));
    playerHandler::player.scale = 2;
    playerHandler::player.orientation = gfx::center;
    
    if(!scene->multiplayer) {
        position_x = blockEngine::getSpawnX();
        position_y = blockEngine::getSpawnY() - player.getHeight() / 2;
    }
    view_x = position_x;
    view_y = position_y;
     
    selectSlot(0);
    player_inventory.open = false;
    
    listening_to = {packets::INVENTORY_CHANGE, packets::SPAWN_POS};
}

#define VELOCITY 20
#define JUMP_VELOCITY 80

static bool key_left = false, key_right = false;

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
        case gfx::KEY_1: selectSlot(0); break;
        case gfx::KEY_2: selectSlot(1); break;
        case gfx::KEY_3: selectSlot(2); break;
        case gfx::KEY_4: selectSlot(3); break;
        case gfx::KEY_5: selectSlot(4); break;
        case gfx::KEY_6: selectSlot(5); break;
        case gfx::KEY_7: selectSlot(6); break;
        case gfx::KEY_8: selectSlot(7); break;
        case gfx::KEY_9: selectSlot(8); break;
        case gfx::KEY_0: selectSlot(9); break;
        case gfx::KEY_E:
            player_inventory.open = !player_inventory.open;
            if(!player_inventory.open && player_inventory.getMouseItem()->item_id != itemEngine::NOTHING) {
                unsigned char result = player_inventory.addItem(player_inventory.getMouseItem()->item_id, player_inventory.getMouseItem()->getStack());
                player_inventory.clearMouseItem();
                packets::packet packet(packets::INVENTORY_SWAP);
                packet << result;
                scene->networking_manager.sendPacket(packet);
            }
            break;
        case gfx::KEY_MOUSE_LEFT: {
            if(hovered) {
                player_inventory.swapWithMouseItem(hovered);
                if(scene->multiplayer) {
                    packets::packet packet(packets::INVENTORY_SWAP);
                    packet << (unsigned char)(hovered - &player_inventory.inventory[0]);
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
       position_y >= blockEngine::world_height * BLOCK_WIDTH - playerHandler::player.getHeight() / 2 ||
       position_x >= blockEngine::world_width * BLOCK_WIDTH - playerHandler::player.getWidth() / 2)
        return true;

    short begin_x = position_x / BLOCK_WIDTH - playerHandler::player.getWidth() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    short end_x = position_x / BLOCK_WIDTH + playerHandler::player.getWidth() / 2 / BLOCK_WIDTH + COLLISION_PADDING;

    short begin_y = position_y / BLOCK_WIDTH - playerHandler::player.getHeight() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    short end_y = position_y / BLOCK_WIDTH + playerHandler::player.getHeight() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
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
            if(blockEngine::getChunkState(x >> 4, y >> 4) != blockEngine::loaded || (gfx::colliding(gfx::rectShape(short(x * BLOCK_WIDTH - playerHandler::view_x + gfx::getWindowWidth() / 2), short(y * BLOCK_WIDTH - playerHandler::view_y + gfx::getWindowHeight() / 2), BLOCK_WIDTH, BLOCK_WIDTH), playerHandler::player.getTranslatedRect()) && !blockEngine::getBlock(x, y).getUniqueBlock().ghost))
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
    if(view_x >= blockEngine::world_width * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
        view_x = blockEngine::world_width * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
    if(view_y >= blockEngine::world_height * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
        view_y = blockEngine::world_height * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
    
    if(scene->multiplayer && (move_x || move_y)) {
        packets::packet packet(packets::PLAYER_MOVEMENT);
        packet << position_x << position_y << (char)/*player.flipped*/ false;
        scene->networking_manager.sendPacket(packet);
    }
    
    // look for items to be picked up
    if(!scene->multiplayer)
        for(unsigned long i = 0; i < itemEngine::items.size(); i++)
            if(abs(itemEngine::items[i].x / 100 + BLOCK_WIDTH / 2  - position_x - playerHandler::player.getWidth() / 2) < 50 && abs(itemEngine::items[i].y / 100 + BLOCK_WIDTH / 2 - position_y - playerHandler::player.getHeight() / 2) < 50 && playerHandler::player_inventory.addItem(itemEngine::items[i].getItemId(), 1) != -1) {
                itemEngine::items[i].destroy();
                itemEngine::items.erase(itemEngine::items.begin() + i);
            }
}

void playerHandler::module::renderItem(inventory::inventoryItem* item, int x, int y, int i) {
    if(itemEngineClient::getUniqueRenderItem(item->item_id).texture.getTexture())
        gfx::render(itemEngineClient::getUniqueRenderItem(item->item_id).texture, gfx::rectShape((short)x, (short)y, 30, 30));
    if(item->getStack() > 1) {
        gfx::image *stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        gfx::render(*stack_texture, x + BLOCK_WIDTH * 2 - stack_texture->getTextureWidth(), y + BLOCK_WIDTH * 2 - stack_texture->getTextureHeight());
    }
}

void playerHandler::module::updateStackTexture(int i) {
    inventory::inventoryItem* item = i == -1 ? playerHandler::player_inventory.getMouseItem() : &playerHandler::player_inventory.inventory[i];
    if(item->stack_changed) {
        gfx::image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        if(item->getStack() > 1)
            stack_texture->setTexture(gfx::renderText(std::to_string(item->getStack()), {255, 255, 255}));
    }
}

void playerHandler::module::render() {
    player.x = position_x - view_x;
    player.y = position_y - view_y;
    gfx::render(player);
    
    select_rect.x = (player_inventory.selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
    gfx::render(select_rect);
    
    gfx::sprite* text_texture = nullptr;
    hovered = nullptr;
    for(int i = -1; i < 20; i++)
        updateStackTexture(i);
    for(int i = 0; i < (player_inventory.open ? 20 : 10); i++) {
        if(gfx::colliding(inventory_slots[i].getTranslatedRect(), gfx::rectShape((short)gfx::getMouseX(), (short)gfx::getMouseY(), 0, 0)) && player_inventory.open) {
            hovered = &player_inventory.inventory[i];
            inventory_slots[i].c = {70, 70, 70};
            if(player_inventory.inventory[i].item_id != itemEngine::NOTHING) {
                text_texture = &itemEngineClient::getUniqueRenderItem(player_inventory.inventory[i].item_id).text_texture;
                text_texture->x = gfx::getMouseX() + 20;
                text_texture->y = gfx::getMouseY() + 20;
                under_text_rect.h = text_texture->getHeight() + 2 * MARGIN;
                under_text_rect.w = text_texture->getWidth() + 2 * MARGIN;
                under_text_rect.x = gfx::getMouseX() + 20 - MARGIN;
                under_text_rect.y = gfx::getMouseY() + 20 - MARGIN;
            }
        }
        else
            inventory_slots[i].c = {100, 100, 100};
        gfx::render(inventory_slots[i]);
        renderItem(&player_inventory.inventory[i], inventory_slots[i].getTranslatedX() + MARGIN / 2, inventory_slots[i].getTranslatedY() + MARGIN / 2, i);
    }
    
    if(text_texture) {
        gfx::render(under_text_rect);
        gfx::render(*text_texture);
    }
    renderItem(player_inventory.getMouseItem(), gfx::getMouseX(), gfx::getMouseY(), -1);
}

void playerHandler::module::selectSlot(char slot) {
    player_inventory.selected_slot = slot;
    if(scene->multiplayer) {
        packets::packet packet(packets::HOTBAR_SELECTION);
        packet << slot;
        scene->networking_manager.sendPacket(packet);
    }
}

void playerHandler::module::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::INVENTORY_CHANGE: {
            char pos = packet.getChar();
            playerHandler::player_inventory.inventory[(int)pos].setStack(packet.getUShort());
            playerHandler::player_inventory.inventory[(int)pos].item_id = (itemEngine::itemType)packet.getUChar();
            break;
        }
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
