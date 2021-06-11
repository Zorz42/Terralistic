//
//  choiceScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#include "choiceScreen.hpp"

choiceScreen::choiceScreen(std::string notification, std::vector<std::string> options, std::string* result) {
    this->notification = notification;
    this->result = result;
    this->options = options;
}

void choiceScreen::init() {
    for (std::string option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }

    notification_sprite.scale = 3;
    notification_sprite.setTexture(gfx::renderText(notification, {255, 255, 255}));
    notification_sprite.orientation = gfx::center;
    
    int combined_width = 0;
    
    for(button& i : buttons) {
        i.gfx_button.scale = 3;
        i.gfx_button.setTexture(gfx::renderText(i.option, {255, 255, 255}));
        i.gfx_button.orientation = gfx::bottom;
        i.gfx_button.y = -20;
        combined_width += i.gfx_button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(button& i : buttons) {
        i.gfx_button.x = curr_x + i.gfx_button.getWidth() / 2;
        curr_x += i.gfx_button.getWidth();
    }
}

void choiceScreen::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT)
        for(button& i : buttons)
            if(i.gfx_button.isHovered()) {
                if(result)
                    *result = i.option;
                gfx::returnFromScene();
                break;
            }
}

void choiceScreen::render() {
    for(button& i : buttons)
        gfx::render(i.gfx_button);
    gfx::render(notification_sprite);
}
