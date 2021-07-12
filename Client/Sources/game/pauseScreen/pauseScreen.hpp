//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include "graphics.hpp"

class pauseScreen : public gfx::sceneModule {
    gfx::button resume_button, quit_button;
    gfx::rect back_rect;
    bool paused = false;
    int x_to_be = 0;
public:
    void init() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
};

#endif /* pauseScreen_hpp */
