//
//  choiceScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/06/2021.
//

#ifndef choiceScreen_hpp
#define choiceScreen_hpp

#include <string>

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#endif

class choiceScreen : public gfx::scene {
    gfx::button yes_button, no_button;
    gfx::sprite notification_sprite;
    std::string notification;
    bool* result;
public:
    choiceScreen(std::string notification, bool* result) : notification(notification), result(result) {}
    
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

#endif /* choiceScreen_hpp */
