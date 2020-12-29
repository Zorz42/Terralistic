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

unsigned int blockSelector::mouseOnMapX() {
    return swl::mouse_x - onMapX(0);
}

unsigned int blockSelector::mouseOnMapY() {
    return swl::mouse_y - onMapY(0);
}

unsigned int blockSelector::onMapX(unsigned int x) {
    return -blockEngine::view_x + swl::window_width / 2 + x;
}

unsigned int blockSelector::onMapY(unsigned int y) {
    return -blockEngine::view_y + swl::window_height / 2 + y;
}

void blockSelector::render() {
    selectedBlockX = mouseOnMapX() / BLOCK_WIDTH;
    selectedBlockY = mouseOnMapY() / BLOCK_WIDTH;
    selectRect.setX(static_cast<short>(onMapX(static_cast<unsigned int>(selectedBlockX * BLOCK_WIDTH))));
    selectRect.setY(static_cast<short>(onMapY(static_cast<unsigned int>(selectedBlockY * BLOCK_WIDTH))));
    selectRect.render();
}

void blockSelector::handleEvent(SDL_Event& event) {
    if(event.type == SDL_MOUSEBUTTONDOWN) {
        if(event.button.button == SDL_BUTTON_LEFT)
            blockEngine::leftClickEvent(static_cast<unsigned short>(selectedBlockX),
                                        static_cast<unsigned short>(selectedBlockY));
        else if(event.button.button == SDL_BUTTON_RIGHT && !swl::colliding(playerHandler::player.getRect(), selectRect.getRect()))
            blockEngine::rightClickEvent(static_cast<unsigned short>(selectedBlockX),
                                         static_cast<unsigned short>(selectedBlockY));
    }
}
