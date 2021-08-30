#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include "graphics.hpp"

class PauseScreen : public gfx::GraphicalModule {
    gfx::Button resume_button, quit_button;
    gfx::Rect back_rect;
    gfx::Rect fade_rect;
    bool paused = false;
    void init() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    void togglePause();
};

#endif
