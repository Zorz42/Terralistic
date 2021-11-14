#include "choiceScreen.hpp"

ChoiceScreen::ChoiceScreen(BackgroundRect* menu_back, std::string question, const std::vector<std::string>& options, std::string* result) : menu_back(menu_back), question(std::move(question)), result(result) {
    for(const std::string& option : options) {
        buttons.emplace_back();
        buttons.back().option = option;
    }
}

void ChoiceScreen::init() {
    question_sprite.scale = 3;
    question_sprite.loadFromText(question);
    question_sprite.orientation = gfx::CENTER;
    
    int combined_width = 0;
    
    for(ChoiceScreenButton& i : buttons) {
        i.gfx_button.scale = 3;
        i.gfx_button.loadFromText(i.option);
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

bool ChoiceScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT)
        for(ChoiceScreenButton& i : buttons)
            if(i.gfx_button.isHovered(getMouseX(), getMouseY())) {
                if(result)
                    *result = i.option;
                returnFromScene();
                return true;
            }
    return false;
}

void ChoiceScreen::render() {
    menu_back->setBackWidth(question_sprite.getWidth() + 100);
    menu_back->renderBack();
    for(ChoiceScreenButton& i : buttons)
        i.gfx_button.render(getMouseX(), getMouseY());
    question_sprite.render();
}
