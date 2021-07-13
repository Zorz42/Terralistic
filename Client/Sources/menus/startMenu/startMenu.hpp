//
//  startMenu.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef startMenu_hpp
#define startMenu_hpp

#undef main

#include "graphics.hpp"

class startMenu : public gfx::Scene {
    gfx::Button singleplayer_button, multiplayer_button, exit_button;
    gfx::Sprite title, version;
    gfx::Image background;
    gfx::Rect back_rect;
#ifdef DEVELOPER_MODE
    gfx::Sprite debug_title;
#endif
public:
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

#endif /* startMenu_hpp */
