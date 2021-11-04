#include "clientInventory.hpp"
#include "resourcePack.hpp"

void ClientInventory::init() {
    manager->packet_event.addListener(this);
    manager->welcome_packet_event.addListener(this);
    
    behind_inventory_rect.orientation = gfx::TOP;
    behind_inventory_rect.setWidth(10 * (BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING * 2) + INVENTORY_UI_SPACING);
    behind_inventory_rect.setY(INVENTORY_UI_SPACING / 2);
    behind_inventory_rect.blur_intensity = BLUR - 2;
    behind_inventory_rect.fill_color.a = TRANSPARENCY;
    behind_inventory_rect.setHeight(2 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    behind_inventory_rect.shadow_intensity = SHADOW_INTENSITY;
    behind_inventory_rect.smooth_factor = 2;
    
    select_rect.orientation = gfx::TOP;
    select_rect.fill_color = GREY;
    select_rect.fill_color.a = TRANSPARENCY;
    select_rect.setY(INVENTORY_UI_SPACING / 2);
    select_rect.setHeight(BLOCK_WIDTH * 4 + 3 * INVENTORY_UI_SPACING);
    select_rect.setWidth(BLOCK_WIDTH * 4 + 3 * INVENTORY_UI_SPACING);
    select_rect.setX(-9 * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING));
    select_rect.smooth_factor = 2;
    
    behind_crafting_rect.setX(INVENTORY_UI_SPACING / 2);
    behind_crafting_rect.setY(INVENTORY_UI_SPACING / 2);
    behind_crafting_rect.setWidth(INVENTORY_ITEM_BACK_RECT_WIDTH + 2 * INVENTORY_UI_SPACING);
    behind_crafting_rect.blur_intensity = BLUR - 2;
    behind_crafting_rect.fill_color.a = TRANSPARENCY;
    behind_crafting_rect.shadow_intensity = SHADOW_INTENSITY;
    behind_crafting_rect.smooth_factor = 2;
    
    under_text_rect.blur_intensity = BLUR - 2;
    under_text_rect.fill_color.a = TRANSPARENCY;
        
    for(int i = 0; i < 10; i++) {
        numbers[i].setColor(WHITE);
        std::string text = "0";
        text[0] += i;
        numbers[i].loadFromText(text);
    }
    
    selected_slot = 0;
}

void ClientInventory::stop() {
    manager->packet_event.removeListener(this);
    manager->welcome_packet_event.removeListener(this);
}

void ClientInventory::render() {
    bool tooltip_active = false;
    
    behind_inventory_rect.setHeight(open ? 8 * BLOCK_WIDTH + 5 * INVENTORY_UI_SPACING : 4 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    behind_inventory_rect.render();
    
    select_rect.setX((2 * (selected_slot - 5) + 1) * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING));
    select_rect.render();
    
    const gfx::Texture* text_texture = nullptr;
    hovered = -1;
    
    for(int i = 0; i < (open ? 20 : 10); i++) {
        int slot_x = (2 * (i - 5 - i / 10 * 10) + 1) * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING) + gfx::getWindowWidth() / 2 - INVENTORY_ITEM_BACK_RECT_WIDTH / 2;
        int slot_y = 1.5 * INVENTORY_UI_SPACING + i / 10 * 2 * (INVENTORY_UI_SPACING + BLOCK_WIDTH * 2);
        
        gfx::RectShape back_rect(slot_x, slot_y, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING);
        gfx::Color color = GREY;
        if(getMouseX() < back_rect.x || getMouseY() < back_rect.y || getMouseX() > back_rect.x + back_rect.w || getMouseY() > back_rect.y + back_rect.h)
            color.a = TRANSPARENCY;
        else if(open) {
            hovered = i;
            if(inventory.getItem(i).type != &ItemTypes::nothing) {
                tooltip_active = true;
                text_texture = &resource_pack->getItemTextTexture((ItemTypeOld)inventory.getItem(i).type->id);
                under_text_rect.setHeight(text_texture->getTextureHeight() * 2 + 2 * INVENTORY_UI_SPACING);
                under_text_rect.setWidth(text_texture->getTextureWidth() * 2 + 2 * INVENTORY_UI_SPACING);
                under_text_rect.setX(getMouseX() + 20 - INVENTORY_UI_SPACING);
                under_text_rect.setY(getMouseY() + 20 - INVENTORY_UI_SPACING);
            }
        }
        
        back_rect.render(color);
        
        renderItem(inventory.getItem(i), slot_x, slot_y);
    }
    
    if(text_texture) {
        under_text_rect.render();
        text_texture->render(2, getMouseX() + 20, getMouseY() + 20);
    }
    
    if(open) {
        renderItem(inventory.getItem(-1), getMouseX(), getMouseY());
        
        hovered_recipe = -1;
        behind_crafting_rect.render();
        
        behind_crafting_rect.setHeight(INVENTORY_UI_SPACING + inventory.getAvailableRecipes().size() * (BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING * 2));
        
        for(int i = 0; i < inventory.getAvailableRecipes().size(); i++) {
            int slot_x = 1.5 * INVENTORY_UI_SPACING;
            int slot_y = 1.5 * INVENTORY_UI_SPACING + i * 2 * (INVENTORY_UI_SPACING + BLOCK_WIDTH * 2);
            
            gfx::RectShape back_rect(slot_x, slot_y, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING);
            gfx::Color color = GREY;
            if(getMouseX() < back_rect.x || getMouseY() < back_rect.y || getMouseX() > back_rect.x + back_rect.w || getMouseY() > back_rect.y + back_rect.h)
                color.a = TRANSPARENCY;
            else
                hovered_recipe = i;
            
            back_rect.render(color);
            
            renderItem(ItemStack(items->getItemTypeById((unsigned char)inventory.getAvailableRecipes()[i]->result_type), inventory.getAvailableRecipes()[i]->result_stack), slot_x, slot_y);
        }
        
        if(hovered_recipe != -1) {
            tooltip_active = true;
            under_text_rect.setX(getMouseX());
            under_text_rect.setY(getMouseY());
            under_text_rect.setWidth(SPACING / 2 + inventory.getAvailableRecipes()[hovered_recipe]->ingredients.size() * (INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING / 2));
            under_text_rect.setHeight(INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING);
            under_text_rect.render();
            int x = getMouseX() + SPACING / 2;
            int y = getMouseY() + SPACING / 2;
            for(auto ingredient : inventory.getAvailableRecipes()[hovered_recipe]->ingredients) {
                gfx::RectShape back_rect(x, y, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING, BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING);
                back_rect.render(GREY);
                renderItem(ItemStack(items->getItemTypeById((unsigned char)ingredient.first), ingredient.second), x, y);
                x += INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING / 2;
            }
        }
    }
    
    under_text_rect.shadow_intensity += ((tooltip_active ? SHADOW_INTENSITY / 2 : 0) - under_text_rect.shadow_intensity) / 3;
}

void ClientInventory::renderItem(ItemStack item, short x, short y) {
    const gfx::Texture& texture = resource_pack->getItemTexture();
    texture.render(4, x + INVENTORY_UI_SPACING / 2, y + INVENTORY_UI_SPACING / 2, resource_pack->getTextureRectangle((ItemTypeOld)item.type->id));
    
    if(item.stack > 1) {
        int stack = item.stack, number_x = x + BLOCK_WIDTH * 4 + INVENTORY_UI_SPACING / 2;
        while(stack) {
            gfx::Texture& number_texture = numbers[stack % 10];
            number_x -= number_texture.getTextureWidth();
            number_texture.render(1, number_x, y + BLOCK_WIDTH * 4 - number_texture.getTextureHeight() + INVENTORY_UI_SPACING / 2);
            stack /= 10;
        }
    }
}

void ClientInventory::selectSlot(char slot) {
    selected_slot = slot;
    sf::Packet packet;
    packet << ClientPacketType::HOTBAR_SELECTION << selected_slot;
    manager->sendPacket(packet);
}

char* ClientInventory::loadFromSerial(char* iter) {
    return inventory.loadFromSerial(iter);
}

void ClientInventory::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::INVENTORY: {
            unsigned short stack;
            unsigned char item_id;
            short pos;
            event.packet >> stack >> item_id >> pos;
            
            inventory.setItem(pos, ItemStack(items->getItemTypeById(item_id), stack));
            break;
        }
        default: break;
    }
}

bool ClientInventory::onKeyDown(gfx::Key key) {
    switch (key) {
        case gfx::Key::NUM1: selectSlot(0); return true;
        case gfx::Key::NUM2: selectSlot(1); return true;
        case gfx::Key::NUM3: selectSlot(2); return true;
        case gfx::Key::NUM4: selectSlot(3); return true;
        case gfx::Key::NUM5: selectSlot(4); return true;
        case gfx::Key::NUM6: selectSlot(5); return true;
        case gfx::Key::NUM7: selectSlot(6); return true;
        case gfx::Key::NUM8: selectSlot(7); return true;
        case gfx::Key::NUM9: selectSlot(8); return true;
        case gfx::Key::NUM0: selectSlot(9); return true;
        case gfx::Key::E:
            open = !open;
            if(!open && inventory.getItem(-1).type != &ItemTypes::nothing) {
                unsigned char result = inventory.addItem(inventory.getItem(-1).type, inventory.getItem(-1).stack);
                inventory.setItem(-1, ItemStack());
                sf::Packet packet;
                packet << ClientPacketType::INVENTORY_SWAP << result;
                manager->sendPacket(packet);
            }
            return true;
        case gfx::Key::MOUSE_LEFT: {
            if(hovered != -1) {
                inventory.swapWithMouseItem(hovered);
                sf::Packet packet;
                packet << ClientPacketType::INVENTORY_SWAP << (unsigned char)hovered;
                manager->sendPacket(packet);
                return true;
            } else if(hovered_recipe != -1) {
                sf::Packet packet;
                packet << ClientPacketType::CRAFT << (unsigned char)hovered_recipe;
                manager->sendPacket(packet);
                return true;
            }
        }
        default: return false;
    }
}

void ClientInventory::onEvent(WelcomePacketEvent &event) {
    if(event.packet_type == WelcomePacketType::INVENTORY) {
        std::vector<char> data = manager->getData();
        inventory.loadFromSerial(&data[0]);
    }
}
