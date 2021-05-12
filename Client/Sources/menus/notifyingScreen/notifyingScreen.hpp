//
//  notifyingScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/05/2021.
//

#ifndef notifyingScreen_hpp
#define notifyingScreen_hpp

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

class notifyingScreen : public gfx::scene {
    gfx::button back_button;
    gfx::sprite notification_sprite;
    std::string notification;
public:
    notifyingScreen(std::string notification) : notification(notification) {}
    
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

#endif /* notifyingScreen_hpp */
