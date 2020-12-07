//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "blockSelector.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"

ogl::rect selectRect(ogl::top_left);

void blockSelector::init() {
    selectRect.fill = false;
    selectRect.setColor(255, 0, 0);
    selectRect.setWidth(BLOCK_WIDTH);
    selectRect.setHeight(BLOCK_WIDTH);
}

int blockSelector::mouseOnMapX() {
    return swl::mouse_x - onMapX(0);
}

int blockSelector::mouseOnMapY() {
    return swl::mouse_y - onMapY(0);
}

int blockSelector::onMapX(unsigned int x) {
    return (int)-blockEngine::view_x + swl::window_width / 2 + x;
}

int blockSelector::onMapY(unsigned int y) {
    return (int)-blockEngine::view_y + swl::window_height / 2 + y;
}

void blockSelector::render() {
    selectedBlockX = mouseOnMapX() / BLOCK_WIDTH;
    selectedBlockY = mouseOnMapY() / BLOCK_WIDTH;
    selectRect.setX(onMapX(selectedBlockX * BLOCK_WIDTH));
    selectRect.setY(onMapY(selectedBlockY * BLOCK_WIDTH));
    selectRect.render();
}

void blockSelector::handleEvent(SDL_Event& event) {
    if(event.type == SDL_MOUSEBUTTONDOWN) {
        if(event.button.button == SDL_BUTTON_LEFT)
            blockEngine::getBlock(selectedBlockX, selectedBlockY) = blockEngine::AIR;
        else if(event.button.button == SDL_BUTTON_RIGHT && blockEngine::getBlock(selectedBlockX, selectedBlockY).block_id == blockEngine::AIR && !swl::colliding(playerHandler::player_rect.getRect(), selectRect.getRect()))
            blockEngine::getBlock(selectedBlockX, selectedBlockY) = blockEngine::GRASS_BLOCK;
        char x[] = {0, 0, 0, -1, 1};
        char y[] = {0, -1, 1, 0, 0};
        for(char i = 0; i < 5; i++)
            blockEngine::getBlock(selectedBlockX + x[i], selectedBlockY + y[i]).update();
    }
}
