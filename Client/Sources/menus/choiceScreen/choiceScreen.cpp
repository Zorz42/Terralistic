#include "choiceScreen.hpp"

#include <utility>

ChoiceScreen::ChoiceScreen(std::string question, const std::vector<std::string>& options, std::string* result) {
    this->question = std::move(question);
    this->result = result;
    for(const std::string& option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }
}

void ChoiceScreen::init() {
    question_sprite.scale = 3;
    question_sprite.renderText(question);
    question_sprite.orientation = gfx::CENTER;
    
    int combined_width = 0;
    
    for(ChoiceScreenButton& i : buttons) {
        i.gfx_button.scale = 3;
        i.gfx_button.renderText(i.option);
        i.gfx_button.orientation = gfx::BOTTOM;
        i.gfx_button.y = -20;
        combined_width += i.gfx_button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(ChoiceScreenButton& i : buttons) {
        i.gfx_button.x = curr_x + i.gfx_button.getWidth() / 2;
        curr_x += i.gfx_button.getWidth();
    }
}

void ChoiceScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT)
        for(ChoiceScreenButton& i : buttons)
            if(i.gfx_button.isHovered()) {
                if(result)
                    *result = i.option;
                gfx::returnFromScene();
                break;
            }
}

void ChoiceScreen::render() {
    for(ChoiceScreenButton& i : buttons)
        i.gfx_button.render();
    question_sprite.render();
}
