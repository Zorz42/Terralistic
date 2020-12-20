//
//  inventoryRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#include "inventoryRenderer.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "blockEngine.hpp"

ogl::rect inventory_slots[10], select_rect;

#define MARGIN 10

void inventoryRenderer::init() {
    for(int i = 0; i < 10; i++) {
        inventory_slots[i].setOrientation(ogl::top);
        inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].setHeight(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setWidth(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setY(MARGIN);
        inventory_slots[i].setX((i - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN);
    }
    
    select_rect.setOrientation(ogl::top);
    select_rect.setColor(50, 50, 50);
    select_rect.setWidth(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setHeight(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setY(MARGIN / 2);
}

void inventoryRenderer::render() {
    select_rect.setX((itemEngine::selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN);
    select_rect.render();
    for(int i = 0; i < 10; i++) {
        inventory_slots[i].render();
        itemEngine::inventory[i].render(inventory_slots[i].getX() + MARGIN / 2, inventory_slots[i].getY() + MARGIN / 2);
    }
}
