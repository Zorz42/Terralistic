//
//  choiceScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#ifndef choiceScreen_hpp
#define choiceScreen_hpp

#include <string>
#include <vector>

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

class choiceScreen : public gfx::scene {
    struct button {
        std::string option;
        gfx::button gfx_button;
    };

    gfx::sprite notification_sprite;
    std::string notification, *result;
    std::vector<button> buttons;
public:
    choiceScreen(std::string notification, const std::vector<std::string>& options, std::string* result = nullptr);

    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

#endif /* choiceScreen_hpp */
