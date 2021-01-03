//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "itemEngine.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"

void itemEngine::init() {
    unique_items = {
        {"nothing",     0,  blockEngine::AIR        },
        {"stone",       99, blockEngine::STONE      },
        {"dirt",        99, blockEngine::DIRT       },
        {"stone_block", 99, blockEngine::STONE_BLOCK},
    };
}

void itemEngine::prepare() {
    selectSlot(0);
    inventory_open = false;
}

void itemEngine::close() {
    items.clear();
}

void itemEngine::updateItems() {
    for(unsigned long i = 0; i < items.size(); i++) {
        items.at(i).update();
        if(abs(items.at(i).x / 100 + BLOCK_WIDTH / 2  - blockEngine::position_x - playerHandler::player.getWidth() / 2) < 50 && abs(items.at(i).y / 100 + BLOCK_WIDTH / 2 - blockEngine::position_y - playerHandler::player.getHeight() / 2) < 50)
            if(addItemToInventory(items.at(i).item_id, 1))
                items.erase(items.begin() + i);
    }
}

void itemEngine::renderItems() {
    for(item& i : items)
        i.draw();
}

void itemEngine::spawnItem(itemType item_id, int x, int y) {
    items.emplace_back(item_id, x, y);
}

bool itemEngine::addItemToInventory(itemType id, int quantity) {
    for(auto & i : inventory)
        if(i.item_id == id) {
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    for(auto & i : inventory)
        if(i.item_id == NOTHING) {
            i.item_id = id;
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    return false;
}

void itemEngine::selectSlot(char slot) {
    selected_slot = slot;
    selected_item = &inventory[(unsigned char)slot];
}

bool itemEngine::handleEvents(SDL_Event &event) {
    if(event.type == SDL_TEXTINPUT) {
        char c = event.text.text[0];
        if(c >= '0' && c <= '9') {
            if(c == '0')
                c = ':';
            selectSlot(c - '1');
            return true;
        }
    } else if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_e)
        inventory_open = !inventory_open;
    return false;
}
