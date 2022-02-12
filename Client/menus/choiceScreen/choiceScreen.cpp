#include "choiceScreen.hpp"

ChoiceScreen::ChoiceScreen(BackgroundRect* menu_back, std::string question, const std::vector<std::string>& options, std::string* result) : menu_back(menu_back), question(std::move(question)), result(result) {
    for(int i = 0; i < options.size(); i++) {
        buttons.emplace_back();
        buttons.back().option = options[i];
    }
}

void ChoiceScreen::init() {
    question_sprite.scale = 2.5;
    question_sprite.loadFromText(question);
    question_sprite.orientation = gfx::CENTER;
    
    int combined_width = 0;
    
    for(int i = 0; i < buttons.size(); i++) {
        buttons[i].gfx_button.scale = 2.5;
        buttons[i].gfx_button.loadFromText(buttons[i].option);
        buttons[i].gfx_button.orientation = gfx::BOTTOM;
        buttons[i].gfx_button.y = -20;
        combined_width += buttons[i].gfx_button.getWidth();
    }
    
    int curr_x = -combined_width / 2;
    
    for(int i = 0; i < buttons.size(); i++) {
        buttons[i].gfx_button.x = curr_x + buttons[i].gfx_button.getWidth() / 2;
        curr_x += buttons[i].gfx_button.getWidth();
    }
}

bool ChoiceScreen::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT)
        for(int i = 0; i < buttons.size(); i++)
            if(buttons[i].gfx_button.isHovered(getMouseX(), getMouseY())) {
                if(result)
                    *result = buttons[i].option;
                returnFromScene();
                return true;
            }
    return false;
}

void ChoiceScreen::render() {
    menu_back->setBackWidth(question_sprite.getWidth() + 100);
    menu_back->renderBack();
    for(int i = 0; i < buttons.size(); i++)
        buttons[i].gfx_button.render(getMouseX(), getMouseY());
    question_sprite.render();
}
