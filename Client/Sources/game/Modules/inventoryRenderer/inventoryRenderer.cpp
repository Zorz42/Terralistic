//
//  inventoryRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 03/04/2021.
//

#include "inventoryRenderer.hpp"
#include "playerHandler.hpp"
#include "renderMap.hpp"

#define MARGIN 10

void inventoryRenderer::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].orientation = gfx::top;
        inventory_slots[i].c = {100, 100, 100};
        inventory_slots[i].h = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].w = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].x = (i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
        inventory_slots[i].y = MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH;
    }
    
    selectSlot(0);
    scene->player_inventory.open = false;
    
    listening_to = {packets::INVENTORY_CHANGE};
}

void inventoryRenderer::render() {
    select_rect.x = (scene->player_inventory.selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
    gfx::render(select_rect);
    
    gfx::sprite* text_texture = nullptr;
    playerHandler::hovered = nullptr;
    for(int i = -1; i < 20; i++)
        updateStackTexture(i);
    
    for(int i = 0; i < (scene->player_inventory.open ? 20 : 10); i++) {
        if(gfx::colliding(inventory_slots[i].getTranslatedRect(), gfx::rectShape((short)gfx::getMouseX(), (short)gfx::getMouseY(), 0, 0)) && scene->player_inventory.open) {
            playerHandler::hovered = &scene->player_inventory.inventory[i];
            inventory_slots[i].c = {70, 70, 70};
            if(scene->player_inventory.inventory[i].item_id != map::itemType::NOTHING) {
                text_texture = &renderMap::getUniqueRenderItem(scene->player_inventory.inventory[i].item_id).text_texture;
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
        renderItem(&scene->player_inventory.inventory[i], inventory_slots[i].getTranslatedX() + MARGIN / 2, inventory_slots[i].getTranslatedY() + MARGIN / 2, i);
    }
    
    if(text_texture) {
        gfx::render(under_text_rect);
        gfx::render(*text_texture);
    }
    renderItem(scene->player_inventory.getMouseItem(), gfx::getMouseX(), gfx::getMouseY(), -1);
}

void inventoryRenderer::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::INVENTORY_CHANGE: {
            char pos = packet.getChar();
            scene->player_inventory.inventory[(int)pos].setStack(packet.getUShort());
            scene->player_inventory.inventory[(int)pos].item_id = (map::itemType)packet.getUChar();
            break;
        }
        default: break;
    }
}

void inventoryRenderer::renderItem(inventory::inventoryItem* item, int x, int y, int i) {
    if(renderMap::getUniqueRenderItem(item->item_id).texture.getTexture())
        gfx::render(renderMap::getUniqueRenderItem(item->item_id).texture, gfx::rectShape((short)x, (short)y, 30, 30));
    if(item->getStack() > 1) {
        gfx::image *stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        gfx::render(*stack_texture, x + BLOCK_WIDTH * 2 - stack_texture->getTextureWidth(), y + BLOCK_WIDTH * 2 - stack_texture->getTextureHeight());
    }
}

void inventoryRenderer::selectSlot(char slot) {
    scene->player_inventory.selected_slot = slot;
    if(scene->multiplayer) {
        packets::packet packet(packets::HOTBAR_SELECTION);
        packet << slot;
        scene->networking_manager.sendPacket(packet);
    }
}

void inventoryRenderer::updateStackTexture(int i) {
    inventory::inventoryItem* item = i == -1 ? scene->player_inventory.getMouseItem() : &scene->player_inventory.inventory[i];
    if(item->stack_changed) {
        gfx::image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        if(item->getStack() > 1)
            stack_texture->setTexture(gfx::renderText(std::to_string(item->getStack()), {255, 255, 255}));
    }
}

void inventoryRenderer::onKeyDown(gfx::key key) {
    switch (key) {
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
        default: break;
    }
}
