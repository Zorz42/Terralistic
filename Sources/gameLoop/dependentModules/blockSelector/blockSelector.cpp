//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "blockSelector.hpp"
#include "inventory.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

ogl::rect selectRect(ogl::top_left);

void blockSelector::init() {
    selectRect.fill = false;
    selectRect.setColor(255, 0, 0);
    selectRect.setWidth(BLOCK_WIDTH);
    selectRect.setHeight(BLOCK_WIDTH);
}

void blockSelector::render() {
    if(!playerHandler::hovered) {
        selected_block_x = (unsigned short)(swl::mouse_x + playerHandler::view_x - swl::window_width / 2) / BLOCK_WIDTH;
        selected_block_y = (unsigned short)(swl::mouse_y + playerHandler::view_y - swl::window_height / 2) / BLOCK_WIDTH;
        selectRect.setX(short(-playerHandler::view_x + swl::window_width / 2 + selected_block_x * BLOCK_WIDTH));
        selectRect.setY(short(-playerHandler::view_y + swl::window_height / 2 + selected_block_y * BLOCK_WIDTH));
        selectRect.render();
    }
}

bool blockSelector::collidingWithPlayer() {
    return swl::colliding(playerHandler::player.getRect(), selectRect.getRect());
}
