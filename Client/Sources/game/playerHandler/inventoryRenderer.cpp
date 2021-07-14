//
//  inventoryRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 03/04/2021.
//

#include "playerHandler.hpp"
#include "textures.hpp"

#define MARGIN 10

void playerHandler::initInventory() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].orientation = gfx::TOP;
        inventory_slots[i].c = {100, 100, 100};
        inventory_slots[i].h = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].w = 2 * BLOCK_WIDTH + MARGIN;
        inventory_slots[i].x = (i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
        inventory_slots[i].y = MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH;
    }
    
    selectSlot(0);
}

void playerHandler::renderInventory() {
    select_rect_inventory.x = (player->player_inventory.selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN;
    select_rect_inventory.render();
    
    const gfx::Image* text_texture = nullptr;
    hovered = nullptr;
    for(int i = -1; i < 20; i++)
        updateStackTexture(i);
    
    for(int i = 0; i < (player->player_inventory.open ? 20 : 10); i++) {
        if(gfx::colliding(inventory_slots[i].getTranslatedRect(), gfx::RectShape((short)gfx::getMouseX(), (short)gfx::getMouseY(), 0, 0)) && player->player_inventory.open) {
            hovered = &player->player_inventory.inventory[i];
            inventory_slots[i].c = {70, 70, 70};
            if(player->player_inventory.inventory[i].item_id != ItemType::NOTHING) {
                text_texture = &getItemTextTexture(player->player_inventory.inventory[i].item_id);
                under_text_rect.h = text_texture->getTextureHeight() * 2 + 2 * MARGIN;
                under_text_rect.w = text_texture->getTextureWidth() * 2 + 2 * MARGIN;
                under_text_rect.x = gfx::getMouseX() + 20 - MARGIN;
                under_text_rect.y = gfx::getMouseY() + 20 - MARGIN;
            }
        }
        else
            inventory_slots[i].c = {100, 100, 100};
        inventory_slots[i].render();
        renderItem(&player->player_inventory.inventory[i], inventory_slots[i].getTranslatedX() + MARGIN / 2, inventory_slots[i].getTranslatedY() + MARGIN / 2, i);
    }
    
    if(text_texture) {
        under_text_rect.render();
        text_texture->render(2, gfx::getMouseX() + 20, gfx::getMouseY() + 20);
    }
    renderItem(player->player_inventory.getMouseItem(), gfx::getMouseX(), gfx::getMouseY(), -1);
}

void playerHandler::onPacketInventory(Packet &packet) {
    switch(packet.getType()) {
        case PacketType::INVENTORY_CHANGE: {
            char pos = packet.get<char>();
            player->player_inventory.inventory[(int)pos].item_id = (ItemType)packet.get<unsigned char>();
            player->player_inventory.inventory[(int)pos].setStack(packet.get<unsigned short>());
            break;
        }
        default: break;
    }
}

void playerHandler::renderItem(clientInventoryItem* item, int x, int y, int i) {
    const gfx::Image& texture = getItemTexture(item->item_id);
    if(texture.getTexture())
        texture.render(4, x, y);
    
    if(item->getStack() > 1) {
        gfx::Image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        stack_texture->render(1, x + BLOCK_WIDTH * 2 - stack_texture->getTextureWidth(), y + BLOCK_WIDTH * 2 - stack_texture->getTextureHeight());
    }
}

void playerHandler::selectSlot(char slot) {
    player->player_inventory.selected_slot = slot;
    Packet packet(PacketType::HOTBAR_SELECTION, sizeof(slot));
    packet << slot;
    manager->sendPacket(packet);
}

void playerHandler::updateStackTexture(int i) {
    clientInventoryItem* item = i == -1 ? player->player_inventory.getMouseItem() : &player->player_inventory.inventory[i];
    if(item->stack_changed) {
        gfx::Image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        if(item->getStack() > 1)
            stack_texture->renderText(std::to_string(item->getStack()), {255, 255, 255});
    }
}

void playerHandler::onKeyDownInventory(gfx::key key) {
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
        case gfx::KEY_E:
            player->player_inventory.open = !player->player_inventory.open;
            if(!player->player_inventory.open && player->player_inventory.getMouseItem()->item_id != ItemType::NOTHING) {
                unsigned char result = player->player_inventory.addItem(player->player_inventory.getMouseItem()->item_id, player->player_inventory.getMouseItem()->getStack());
                player->player_inventory.clearMouseItem();
                Packet packet(PacketType::INVENTORY_SWAP, sizeof(result));
                packet << result;
                manager->sendPacket(packet);
            }
            break;
        case gfx::KEY_MOUSE_LEFT: {
            if(hovered) {
                player->player_inventory.swapWithMouseItem(hovered);
                Packet packet(PacketType::INVENTORY_SWAP, sizeof(unsigned char));
                packet << (unsigned char)(hovered - &player->player_inventory.inventory[0]);
                manager->sendPacket(packet);
            }
            break;
        }
        default: break;
    }
}
