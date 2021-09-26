#include "clientInventory.hpp"
#include "resourcePack.hpp"

ClientInventory::ClientInventory(NetworkingManager* manager, ResourcePack* resource_pack) : manager(manager), resource_pack(resource_pack), mouse_item(resource_pack) {
    for(ClientInventoryItem& item : inventory)
        item = ClientInventoryItem(resource_pack);
}

void ClientInventory::init() {
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
    
    selectSlot(0);
}

void ClientInventory::render() {
    bool tooltip_active = false;
    
    behind_inventory_rect.setHeight(open ? 8 * BLOCK_WIDTH + 5 * INVENTORY_UI_SPACING : 4 * BLOCK_WIDTH + 3 * INVENTORY_UI_SPACING);
    behind_inventory_rect.render();
    
    select_rect.setX((2 * (selected_slot - 5) + 1) * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING));
    select_rect.render();
    
    const gfx::Texture* text_texture = nullptr;
    hovered = nullptr;
    inventory_hovered = false;
    
    for(int i = 0; i < (open ? 20 : 10); i++) {
        if(inventory[i].isHovered(mouse_x, mouse_y)) {
            inventory_hovered = true;
            if(open) {
                hovered = &inventory[i];
                if(inventory[i].type != ItemType::NOTHING) {
                    tooltip_active = true;
                    text_texture = &resource_pack->getItemTextTexture(inventory[i].type);
                    under_text_rect.setHeight(text_texture->getTextureHeight() * 2 + 2 * INVENTORY_UI_SPACING);
                    under_text_rect.setWidth(text_texture->getTextureWidth() * 2 + 2 * INVENTORY_UI_SPACING);
                    under_text_rect.setX(mouse_x + 20 - INVENTORY_UI_SPACING);
                    under_text_rect.setY(mouse_y + 20 - INVENTORY_UI_SPACING);
                }
            }
        }
        inventory[i].x = (2 * (i - 5 - i / 10 * 10) + 1) * (BLOCK_WIDTH * 2 + INVENTORY_UI_SPACING) + gfx::getWindowWidth() / 2 - INVENTORY_ITEM_BACK_RECT_WIDTH / 2;
        inventory[i].y = 1.5 * INVENTORY_UI_SPACING + i / 10 * 2 * (INVENTORY_UI_SPACING + BLOCK_WIDTH * 2);
        inventory[i].renderWithBack(mouse_x, mouse_y);
    }
    
    if(text_texture) {
        under_text_rect.render();
        text_texture->render(2, mouse_x + 20, mouse_y + 20);
    }
    
    if(open) {
        crafting_hovered = -1;
        behind_crafting_rect.render();
        for(int i = 0; i < available_recipes.size(); i++) {
            available_recipes[i]->render(mouse_x, mouse_y);
            if(available_recipes[i]->isHovered(mouse_x, mouse_y))
                crafting_hovered = i;
        }
    
        mouse_item.x = mouse_x;
        mouse_item.y = mouse_y;
        mouse_item.render();
        
        if(crafting_hovered != -1) {
            tooltip_active = true;
            under_text_rect.setX(mouse_x);
            under_text_rect.setY(mouse_y);
            under_text_rect.setWidth(SPACING / 2 + available_recipes[crafting_hovered]->recipe->ingredients.size() * (INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING / 2));
            under_text_rect.setHeight(INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING);
            under_text_rect.render();
            available_recipes[crafting_hovered]->renderIngredients(mouse_x, mouse_y, mouse_x, mouse_y);
        }
    }
    
    under_text_rect.shadow_intensity += ((tooltip_active ? SHADOW_INTENSITY / 2 : 0) - under_text_rect.shadow_intensity) / 3;
}

void ClientInventory::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::INVENTORY: {
            unsigned short stack;
            unsigned char item_id;
            short pos;
            event.packet >> stack >> item_id >> pos;
            
            ClientInventoryItem& item = pos == -1 ? mouse_item : inventory[(int)pos];
            
            item.type = (ItemType)item_id;
            item.setStack(stack);
            break;
        }
        case PacketType::RECIPE_AVAILABILTY_CHANGE: {
            for(DisplayRecipe* recipe : available_recipes)
                delete recipe;
            available_recipes.clear();
            unsigned short num_of_recipes;
            event.packet >> num_of_recipes;
            int y = 1.5 * INVENTORY_UI_SPACING;
            for(int i = 0; i < num_of_recipes; i++) {
                unsigned short index;
                event.packet >> index;
                available_recipes.emplace_back(new DisplayRecipe(&getRecipes()[index], resource_pack, 1.5 * INVENTORY_UI_SPACING, y));
                y += INVENTORY_ITEM_BACK_RECT_WIDTH + INVENTORY_UI_SPACING;
            }
            if(available_recipes.empty())
                behind_crafting_rect.setHeight(0);
            else
                behind_crafting_rect.setHeight(y - INVENTORY_UI_SPACING / 2);
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
            } else if(crafting_hovered != -1) {
                sf::Packet packet;
                packet << PacketType::CRAFT << (unsigned char)crafting_hovered;
                manager->sendPacket(packet);
            }
            break;
        }
        default: break;
    }
}

char ClientInventory::addItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].type == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].type == ItemType::NOTHING) {
            inventory[i].type = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

void ClientInventory::swapWithMouseItem(ClientInventoryItem* item) {
    ClientInventoryItem temp(resource_pack);
    temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

void ClientInventory::clearMouseItem() {
    mouse_item.type = ItemType::NOTHING;
    mouse_item.setStack(0);
}
