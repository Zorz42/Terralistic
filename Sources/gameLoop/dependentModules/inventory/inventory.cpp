//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#include "inventory.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"

ogl::rect inventory_slots[20], select_rect, under_text_rect(ogl::top_left);

#define MARGIN 10

void inventory::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].setOrientation(ogl::top);
        inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].setHeight(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setWidth(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setY(MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH);
        inventory_slots[i].setX(short((i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    }
    
    select_rect.setOrientation(ogl::top);
    select_rect.setColor(50, 50, 50);
    select_rect.setWidth(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setHeight(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setY(MARGIN / 2);
    
    under_text_rect.setColor(0, 0, 0);
}

#define MARGIN 10

void inventory::prepare() {
    selectSlot(0);
    inventory_open = false;
}

void inventory::render() {
    select_rect.setX(short((inventory::selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    select_rect.render();
    ogl::texture* text_texture = nullptr;
    hovered = nullptr;
    for(int i = 0; i < (inventory::inventory_open ? 20 : 10); i++) {
        if(swl::colliding(inventory_slots[i].getRect(), {swl::mouse_x, swl::mouse_y, 0, 0}) && inventory::inventory_open) {
            hovered = &player_inventory[i];
            inventory_slots[i].setColor(70, 70, 70);
            if(inventory::player_inventory[i].item_id != itemEngine::NOTHING) {
                text_texture = &inventory::player_inventory[i].getUniqueItem().text_texture;
                text_texture->setX(swl::mouse_x + 20);
                text_texture->setY(swl::mouse_y + 20);
                under_text_rect.setHeight(text_texture->getHeight() + 2 * MARGIN);
                under_text_rect.setWidth(text_texture->getWidth() + 2 * MARGIN);
                under_text_rect.setX(swl::mouse_x + 20 - MARGIN);
                under_text_rect.setY(swl::mouse_y + 20 - MARGIN);
            }
        }
        else
            inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].render();
        inventory::player_inventory[i].render(inventory_slots[i].getX() + MARGIN / 2, inventory_slots[i].getY() + MARGIN / 2);
    }
    if(text_texture) {
        under_text_rect.render();
        text_texture->render();
    }
    mouse_item.render(swl::mouse_x, swl::mouse_y);
}

void inventory::handleEvents(SDL_Event &event) {
    if(event.type == SDL_TEXTINPUT) {
        char c = event.text.text[0];
        if(c >= '0' && c <= '9') {
            if(c == '0')
                c = ':';
            selectSlot(c - '1');
        }
    } else if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_e) {
        inventory_open = !inventory_open;
        if(!inventory_open && mouse_item.item_id != itemEngine::NOTHING) {
            addItemToInventory(mouse_item.item_id, mouse_item.getStack());
            mouse_item.item_id = itemEngine::NOTHING;
            mouse_item.setStack(0);
        }
    } else if(hovered && event.type == SDL_MOUSEBUTTONDOWN && event.button.button == SDL_BUTTON_LEFT) {
        inventoryItem temp = *hovered;
        *hovered = mouse_item;
        mouse_item = temp;
    }
}

bool inventory::addItemToInventory(itemEngine::itemType id, int quantity) {
    for(auto & i : player_inventory)
        if(i.item_id == id) {
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    for(auto & i : player_inventory)
        if(i.item_id == itemEngine::NOTHING) {
            i.item_id = id;
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    return false;
}

void inventory::selectSlot(char slot) {
    selected_slot = slot;
    selected_item = &player_inventory[(unsigned char)slot];
}
