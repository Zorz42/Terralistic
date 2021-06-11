//
//  choiceScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#include "choiceScreen.hpp"

choiceScreen::choiceScreen(std::string notification, std::vector<std::string> options, std::string* result) : notification(notification), result(result) {
    for(std::string option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }
}

void choiceScreen::init() {
    notification_sprite.scale = 3;
    notification_sprite.setTexture(gfx::renderText(notification, {255, 255, 255}));
    notification_sprite.orientation = gfx::center;
    
    int combined_width = 0;
    
    for(button& i : buttons) {
        i.button.scale = 3;
        i.button.setTexture(gfx::renderText(i.option, {255, 255, 255}));
        i.button.orientation = gfx::bottom;
        i.button.y = -20;
        combined_width += i.button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(button& i : buttons) {
        i.button.x = curr_x + i.button.getWidth() / 2;
        curr_x += i.button.getWidth();
    }
}

void choiceScreen::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT)
        for(button& i : buttons)
            if(i.button.isHovered()) {
                if(result)
                    *result = i.option;
                gfx::returnFromScene();
                break;
            }
}

void choiceScreen::render() {
    for(button& i : buttons)
        gfx::render(i.button);
    gfx::render(notification_sprite);
}
