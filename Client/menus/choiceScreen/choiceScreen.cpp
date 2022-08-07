#include "choiceScreen.hpp"

ChoiceScreen::ChoiceScreen(BackgroundRect* menu_back, std::string question, const std::vector<std::string>& options, std::string* result) : gfx::Scene("ChoiceScreen"), menu_back(menu_back), question(std::move(question)), result(result) {
    for(const auto & option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }
}

void ChoiceScreen::init() {
    question_sprite.scale = 3;
    question_sprite.loadFromSurface(gfx::textToSurface(question));
    question_sprite.orientation = gfx::CENTER;
    
    int combined_width = 0;
    
    for(auto & button : buttons) {
        button.gfx_button.scale = 3;
        button.gfx_button.loadFromSurface(gfx::textToSurface(button.option));
        button.gfx_button.orientation = gfx::BOTTOM;
        button.gfx_button.y = -20;
        combined_width += button.gfx_button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(auto & button : buttons) {
        button.gfx_button.x = curr_x + button.gfx_button.getWidth() / 2;
        curr_x += button.gfx_button.getWidth();
    }
}

bool ChoiceScreen::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT)
        for(auto & button : buttons)
            if(button.gfx_button.isHovered(getMouseX(), getMouseY())) {
                if(result)
                    *result = button.option;
                returnFromScene();
                return true;
            }
    return false;
}

void ChoiceScreen::render() {
    menu_back->setBackWidth(question_sprite.getWidth() + 100);
    menu_back->renderBack();
    for(auto & button : buttons)
        button.gfx_button.render(getMouseX(), getMouseY());
    question_sprite.render();
}
