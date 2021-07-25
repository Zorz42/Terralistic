//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include "graphics.hpp"

class pauseScreen : public gfx::GraphicalModule {
    gfx::Button resume_button, quit_button;
    gfx::Rect back_rect;
    gfx::Rect fade_rect;
    bool paused = false;
    int x_to_be = 0;
public:
    void init() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
};

#endif /* pauseScreen_hpp */
