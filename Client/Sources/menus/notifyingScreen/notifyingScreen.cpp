//
//  notifyingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/05/2021.
//

#include "notifyingScreen.hpp"

void notifyingScreen::init() {
    notification_sprite.scale = 3;
    notification_sprite.setTexture(gfx::renderText(notification, {255, 255, 255}));
    notification_sprite.orientation = gfx::center;
    
    back_button.scale = 3;
    back_button.setTexture(gfx::renderText("Back", {255, 255, 255}));
    back_button.orientation = gfx::bottom;
    back_button.y = -20;
}

void notifyingScreen::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
}

void notifyingScreen::render() {
    gfx::render(back_button);
    gfx::render(notification_sprite);
}
