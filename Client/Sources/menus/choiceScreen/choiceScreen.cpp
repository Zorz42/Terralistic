//
//  choiceScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#include "choiceScreen.hpp"

void choiceScreen::init() {
    notification_sprite.scale = 3;
    notification_sprite.setTexture(gfx::renderText(notification, {255, 255, 255}));
    notification_sprite.orientation = gfx::center;
    
    yes_button.scale = 3;
    yes_button.setTexture(gfx::renderText("Yes", {255, 255, 255}));
    yes_button.orientation = gfx::bottom;
    yes_button.y = -20;
    yes_button.x = -yes_button.getWidth() / 2;
    
    no_button.scale = 3;
    no_button.setTexture(gfx::renderText("No", {255, 255, 255}));
    no_button.orientation = gfx::bottom;
    no_button.y = -20;
    no_button.x = no_button.getWidth() / 2;
}

void choiceScreen::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && (yes_button.isHovered() || no_button.isHovered()))
        *result = yes_button.isHovered();
        gfx::returnFromScene();
}

void choiceScreen::render() {
    gfx::render(yes_button);
    gfx::render(no_button);
    gfx::render(notification_sprite);
}
