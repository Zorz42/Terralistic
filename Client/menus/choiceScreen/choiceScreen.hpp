#ifndef choiceScreen_hpp
#define choiceScreen_hpp

#include <string>
#include <vector>
#include "graphics.hpp"
#include "menuBack.hpp"

struct ChoiceScreenButton {
    std::string option;
    gfx::Button gfx_button;
};

class ChoiceScreen : public gfx::Scene {
    gfx::Sprite question_sprite;
    std::string question, *result;
    std::vector<ChoiceScreenButton> buttons;
    MenuBack* menu_back;
public:
    ChoiceScreen(MenuBack* menu_back, std::string question, const std::vector<std::string>& options, std::string* result = nullptr);

    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
};

#endif
