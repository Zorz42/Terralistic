#ifndef choiceScreen_hpp
#define choiceScreen_hpp

#include <string>
#include <vector>

#include "graphics.hpp"

class choiceScreen : public gfx::Scene {
    struct button {
        std::string option;
        gfx::Button gfx_button;
    };

    gfx::Sprite question_sprite;
    std::string question, *result;
    std::vector<button> buttons;
public:
    choiceScreen(std::string question, const std::vector<std::string>& options, std::string* result = nullptr);

    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
};

#endif /* choiceScreen_hpp */
