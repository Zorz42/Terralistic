#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include "graphics.hpp"

class PauseScreen : public gfx::GraphicalModule {
    gfx::Button resume_button, quit_button;
    gfx::Rect back_rect;
    gfx::Rect fade_rect;
    bool paused = false;
    int x_to_be = 0;
    void init() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
};

#endif
