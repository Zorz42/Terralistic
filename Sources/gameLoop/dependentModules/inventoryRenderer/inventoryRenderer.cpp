//
//  inventoryRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#include "inventoryRenderer.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "blockEngine.hpp"
#include "itemEngine.hpp"
#include "singleWindowLibrary.hpp"

ogl::rect inventory_slots[20], select_rect, under_text_rect(ogl::top_left);

#define MARGIN 10

void inventoryRenderer::init() {
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].setOrientation(ogl::top);
        inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].setHeight(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setWidth(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setY(MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH);
        inventory_slots[i].setX(
                short((i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    }
    
    select_rect.setOrientation(ogl::top);
    select_rect.setColor(50, 50, 50);
    select_rect.setWidth(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setHeight(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setY(MARGIN / 2);
    
    under_text_rect.setColor(0, 0, 0);
}

#define MARGIN 10

void inventoryRenderer::render() {
    select_rect.setX(short((itemEngine::selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    select_rect.render();
    ogl::texture* text_texture = nullptr;
    for(int i = 0; i < (itemEngine::inventory_open ? 20 : 10); i++) {
        if(swl::colliding(inventory_slots[i].getRect(), {swl::mouse_x, swl::mouse_y, 0, 0}) && itemEngine::inventory_open && itemEngine::inventory[i].item_id != itemEngine::NOTHING) {
            inventory_slots[i].setColor(70, 70, 70);
            text_texture = &itemEngine::inventory[i].getUniqueItem().text_texture;
            text_texture->setX(swl::mouse_x + 20);
            text_texture->setY(swl::mouse_y + 20);
            under_text_rect.setHeight(text_texture->getHeight() + 2 * MARGIN);
            under_text_rect.setWidth(text_texture->getWidth() + 2 * MARGIN);
            under_text_rect.setX(swl::mouse_x + 20 - MARGIN);
            under_text_rect.setY(swl::mouse_y + 20 - MARGIN);
        }
        else
            inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].render();
        itemEngine::inventory[i].render(inventory_slots[i].getX() + MARGIN / 2, inventory_slots[i].getY() + MARGIN / 2);
    }
    if(text_texture) {
        under_text_rect.render();
        text_texture->render();
    }
}
