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

class startMenu : public gfx::scene {
    gfx::button singleplayer_button, multiplayer_button, exit_button;
    gfx::sprite title, version;
    gfx::image background;
    gfx::rect back_rect;
#ifdef DEVELOPER_MODE
    gfx::sprite debug_title;
#endif
public:
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

#endif /* startMenu_hpp */
