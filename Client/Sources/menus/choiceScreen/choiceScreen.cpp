//
//  choiceScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#include "choiceScreen.hpp"

#include <utility>

choiceScreen::choiceScreen(std::string question, const std::vector<std::string>& options, std::string* result) {
    this->question = std::move(question);
    this->result = result;
    for(const std::string& option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }
}

void choiceScreen::init() {
    question_sprite.scale = 3;
    question_sprite.renderText(question, {255, 255, 255});
    question_sprite.orientation = gfx::CENTER;
    
    int combined_width = 0;
    
    for(button& i : buttons) {
        i.gfx_button.scale = 3;
        i.gfx_button.renderText(i.option, {255, 255, 255});
        i.gfx_button.orientation = gfx::BOTTOM;
        i.gfx_button.y = -20;
        combined_width += i.gfx_button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(button& i : buttons) {
        i.gfx_button.x = curr_x + i.gfx_button.getWidth() / 2;
        curr_x += i.gfx_button.getWidth();
    }
}

void choiceScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT)
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
        i.gfx_button.render();
    question_sprite.render();
}
