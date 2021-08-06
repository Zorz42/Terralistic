#include "clientInventory.hpp"
#include "resourcePack.hpp"

ClientInventory::ClientInventory(networkingManager* manager, ResourcePack* resource_pack) : manager(manager), resource_pack(resource_pack), mouse_item(resource_pack) {
    for(ClientInventoryItem& item : inventory)
        item = ClientInventoryItem(resource_pack);
}

void ClientInventory::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].orientation = gfx::TOP;
        inventory_slots[i].setHeight(32 + INVENTORY_UI_SPACING);
        inventory_slots[i].setWidth(32 + INVENTORY_UI_SPACING);
        inventory_slots[i].setX((2 * (i - 5 - i / 10 * 10) + 1) * (BLOCK_WIDTH + INVENTORY_UI_SPACING));
        inventory_slots[i].setY(1.5 * INVENTORY_UI_SPACING + i / 10 * 2 * (INVENTORY_UI_SPACING + BLOCK_WIDTH));
    }
    
    behind_inventory_rect.orientation = gfx::TOP;
    behind_inventory_rect.setWidth(10 * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING * 2) + INVENTORY_UI_SPACING);
    behind_inventory_rect.setY(INVENTORY_UI_SPACING / 2);
    behind_inventory_rect.blur_intensity = BLUR - 2;
    behind_inventory_rect.c.a = TRANSPARENCY;
    behind_inventory_rect.setHeight(2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    behind_inventory_rect.shadow_intensity = SHADOW_INTENSITY;
    behind_inventory_rect.smooth_factor = 2;
    
    select_rect.orientation = gfx::TOP;
    select_rect.c = GREY;
    select_rect.c.a = TRANSPARENCY;
    select_rect.setY(INVENTORY_UI_SPACING / 2);
    select_rect.setHeight(2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    select_rect.setWidth(2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    select_rect.setX(-9 * (BLOCK_WIDTH + INVENTORY_UI_SPACING));
    select_rect.smooth_factor = 2;
    
    crafting_back.setX(INVENTORY_UI_SPACING / 2);
    crafting_back.setY(INVENTORY_UI_SPACING / 2);
    crafting_back.setWidth(32 + 3 * INVENTORY_UI_SPACING);
    crafting_back.blur_intensity = BLUR - 2;
    crafting_back.c.a = TRANSPARENCY;
    crafting_back.shadow_intensity = SHADOW_INTENSITY;
    crafting_back.smooth_factor = 2;
    
    selectSlot(0);
}

void ClientInventory::render() {
    behind_inventory_rect.setHeight(open ? 4 * BLOCK_WIDTH + 5 * INVENTORY_UI_SPACING : 2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    behind_inventory_rect.render();
    
    select_rect.setX((2 * (selected_slot - 5) + 1) * (BLOCK_WIDTH + INVENTORY_UI_SPACING));
    select_rect.render();
    
    const gfx::Image* text_texture = nullptr;
    hovered = nullptr;
    inventory_hovered = false;
    
    for(int i = 0; i < (open ? 20 : 10); i++) {
        if(gfx::colliding(inventory_slots[i].getTranslatedRect(), gfx::RectShape((short)gfx::getMouseX(), (short)gfx::getMouseY(), 0, 0))) {
            inventory_hovered = true;
            if (open) {
                hovered = &inventory[i];
                inventory_slots[i].c = {70, 70, 70};
                if(inventory[i].type != ItemType::NOTHING) {
                    text_texture = &resource_pack->getItemTextTexture(inventory[i].type);
                    under_text_rect.setHeight(text_texture->getTextureHeight() * 2 + 2 * INVENTORY_UI_SPACING);
                    under_text_rect.setWidth(text_texture->getTextureWidth() * 2 + 2 * INVENTORY_UI_SPACING);
                    under_text_rect.setX(gfx::getMouseX() + 20 - INVENTORY_UI_SPACING);
                    under_text_rect.setY(gfx::getMouseY() + 20 - INVENTORY_UI_SPACING);
                }
            }
        } else {
            inventory_slots[i].c = WHITE;
            inventory_slots[i].c.a = TRANSPARENCY;
        }
        inventory_slots[i].render();
        inventory[i].render(inventory_slots[i].getTranslatedX() + INVENTORY_UI_SPACING / 2, inventory_slots[i].getTranslatedY() + INVENTORY_UI_SPACING / 2);
    }
    
    if(text_texture) {
        under_text_rect.render();
        text_texture->render(2, gfx::getMouseX() + 20, gfx::getMouseY() + 20);
    }
    mouse_item.render(gfx::getMouseX(), gfx::getMouseY());
    
    if(open) {
        crafting_back.render();
        for(DisplayRecipe* recipe : available_recipes)
            recipe->render();
    }
}

void ClientInventory::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::INVENTORY_CHANGE: {
            unsigned short stack;
            unsigned char item_id;
            unsigned char pos;
            event.packet >> stack >> item_id >> pos;
            
            inventory[(int)pos].type = (ItemType)item_id;
            inventory[(int)pos].setStack(stack);
            break;
        }
        case PacketType::RECIPE_AVAILABILTY_CHANGE: {
            for(DisplayRecipe* recipe : available_recipes)
                delete recipe;
            available_recipes.clear();
            int back_height = INVENTORY_UI_SPACING;
            for(int y = 1.5 * INVENTORY_UI_SPACING; !event.packet.endOfPacket(); y += 32 + INVENTORY_UI_SPACING * 2) {
                unsigned short index;
                event.packet >> index;
                available_recipes.emplace_back(new DisplayRecipe(&getRecipes()[index], resource_pack, 1.5 * INVENTORY_UI_SPACING, y));
                available_recipes.back()->updateResult();
                back_height += 32 + 2 * INVENTORY_UI_SPACING;
            }
            crafting_back.setHeight(back_height);
            break;
        }
        default: break;
    }
}

void ClientInventory::selectSlot(char slot) {
    selected_slot = slot;
    sf::Packet packet;
    packet << PacketType::HOTBAR_SELECTION << selected_slot;
    manager->sendPacket(packet);
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
            if(!open && mouse_item.type != ItemType::NOTHING) {
                unsigned char result = addItem(mouse_item.type, mouse_item.getStack());
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
