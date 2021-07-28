#include "clientInventory.hpp"
#include "resourcePack.hpp"

#define INVENTORY_UI_SPACING 10

void ClientInventory::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].orientation = gfx::TOP;
        inventory_slots[i].c = GREY;
        inventory_slots[i].h = 2 * BLOCK_WIDTH + INVENTORY_UI_SPACING;
        inventory_slots[i].w = 2 * BLOCK_WIDTH + INVENTORY_UI_SPACING;
        inventory_slots[i].x = (2 * (i - 5 - i / 10 * 10) + 1) * (BLOCK_WIDTH + INVENTORY_UI_SPACING);
        inventory_slots[i].y = INVENTORY_UI_SPACING + i / 10 * 2 * (INVENTORY_UI_SPACING + BLOCK_WIDTH);
    }
    
    behind_inventory_rect.orientation = gfx::TOP;
    behind_inventory_rect.w = 10 * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING * 2) + INVENTORY_UI_SPACING;
    behind_inventory_rect.c = BLACK;
    
    under_text_rect.c = BLACK;
    
    selectSlot(0);
}

void ClientInventory::render() {
    behind_inventory_rect.h = open ? 4 * BLOCK_WIDTH + 5 * INVENTORY_UI_SPACING : 2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING;
    behind_inventory_rect.render();
    
    const gfx::Image* text_texture = nullptr;
    hovered = nullptr;
    for(int i = -1; i < 20; i++)
        updateStackTexture(i);
    inventory_hovered = false;
    
    for(int i = 0; i < (open ? 20 : 10); i++) {
        if(gfx::colliding(inventory_slots[i].getTranslatedRect(), gfx::RectShape((short)gfx::getMouseX(), (short)gfx::getMouseY(), 0, 0))) {
            inventory_hovered = true;
            if (open) {
                hovered = &inventory[i];
                inventory_slots[i].c = {70, 70, 70};
                if(inventory[i].item_id != ItemType::NOTHING) {
                    text_texture = &resource_pack->getItemTextTexture(inventory[i].item_id);
                    under_text_rect.h = text_texture->getTextureHeight() * 2 + 2 * INVENTORY_UI_SPACING;
                    under_text_rect.w = text_texture->getTextureWidth() * 2 + 2 * INVENTORY_UI_SPACING;
                    under_text_rect.x = gfx::getMouseX() + 20 - INVENTORY_UI_SPACING;
                    under_text_rect.y = gfx::getMouseY() + 20 - INVENTORY_UI_SPACING;
                }
            }
        } else
            inventory_slots[i].c = GREY;
        inventory_slots[i].render();
        renderItem(&inventory[i], inventory_slots[i].getTranslatedX() + INVENTORY_UI_SPACING / 2, inventory_slots[i].getTranslatedY() + INVENTORY_UI_SPACING / 2, i);
    }
    
    if(text_texture) {
        under_text_rect.render();
        text_texture->render(2, gfx::getMouseX() + 20, gfx::getMouseY() + 20);
    }
    renderItem(&mouse_item, gfx::getMouseX(), gfx::getMouseY(), -1);
}

void ClientInventory::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::INVENTORY_CHANGE: {
            unsigned short stack;
            unsigned char item_id;
            unsigned char pos;
            event.packet >> stack >> item_id >> pos;
            
            inventory[(int)pos].item_id = (ItemType)item_id;
            inventory[(int)pos].setStack(stack);
            break;
        }
        default: break;
    }
}

void ClientInventory::renderItem(ClientInventoryItem* item, int x, int y, int i) {
    const gfx::Image& texture = resource_pack->getItemTexture(item->item_id);
    texture.render(4, x, y);
    
    if(item->getStack() > 1) {
        gfx::Image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        stack_texture->render(1, x + BLOCK_WIDTH * 2 - stack_texture->getTextureWidth(), y + BLOCK_WIDTH * 2 - stack_texture->getTextureHeight());
    }
}

void ClientInventory::selectSlot(char slot) {
    selected_slot = slot;
    sf::Packet packet;
    packet << PacketType::HOTBAR_SELECTION << slot;
    manager->sendPacket(packet);
}

void ClientInventory::updateStackTexture(int i) {
    ClientInventoryItem* item = i == -1 ? &mouse_item : &inventory[i];
    if(item->stack_changed) {
        gfx::Image* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        if(item->getStack() > 1)
            stack_texture->renderText(std::to_string(item->getStack()));
    }
}

void ClientInventory::onKeyDown(gfx::Key key) {
    switch (key) {
        case gfx::Key::NUM1: selectSlot(0); break;
        case gfx::Key::NUM2: selectSlot(1); break;
        case gfx::Key::NUM3: selectSlot(2); break;
        case gfx::Key::NUM4: selectSlot(3); break;
        case gfx::Key::NUM5: selectSlot(4); break;
        case gfx::Key::NUM6: selectSlot(5); break;
        case gfx::Key::NUM7: selectSlot(6); break;
        case gfx::Key::NUM8: selectSlot(7); break;
        case gfx::Key::NUM9: selectSlot(8); break;
        case gfx::Key::NUM0: selectSlot(9); break;
        case gfx::Key::E:
            open = !open;
            if(!open && mouse_item.item_id != ItemType::NOTHING) {
                unsigned char result = addItem(mouse_item.item_id, mouse_item.getStack());
                clearMouseItem();
                sf::Packet packet;
                packet << PacketType::INVENTORY_SWAP << result;
                manager->sendPacket(packet);
            }
            break;
        case gfx::Key::MOUSE_LEFT: {
            if(hovered) {
                swapWithMouseItem(hovered);
                sf::Packet packet;
                packet << PacketType::INVENTORY_SWAP << (unsigned char)(hovered - &inventory[0]);
                manager->sendPacket(packet);
            }
            break;
        }
        default: break;
    }
}
