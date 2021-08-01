#include "pauseScreen.hpp"
#include <cmath>

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.renderText("Resume");
    resume_button.y = SPACING;

    quit_button.scale = 3;
    quit_button.renderText("Leave Game");
    quit_button.y = short(resume_button.getHeight() + 2 * SPACING);
    
    back_rect.w = quit_button.getWidth() + 2 * SPACING;
    back_rect.c.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    
    x_to_be = -back_rect.w;
    back_rect.x = x_to_be;
}

void PauseScreen::render() {
    if(back_rect.x != -back_rect.w || x_to_be != -back_rect.w) {
        back_rect.x += std::floor(float(x_to_be - back_rect.x) / 2.0f);
        resume_button.x = back_rect.x + SPACING;
        quit_button.x = back_rect.x + SPACING;
        back_rect.h = gfx::getWindowHeight();
        fade_rect.w = gfx::getWindowWidth();
        fade_rect.h = gfx::getWindowHeight();
        fade_rect.c.a = float(back_rect.w + back_rect.x) / (float)back_rect.w * 70;
        fade_rect.blur_intensity = float(back_rect.w + back_rect.x) / (float)back_rect.w * BLUR;
        if(fade_rect.blur_intensity < 0.5f)
            fade_rect.blur_intensity = 0;
        fade_rect.render();
        back_rect.render();
        resume_button.render();
        quit_button.render();
    }
}

void PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        paused = !paused;
        x_to_be = !paused * (-back_rect.w);
        disable_events = paused;
    }
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered()) {
            paused = false;
            x_to_be = !paused * (-back_rect.w);
            disable_events = paused;
        }
        else if(quit_button.isHovered()) {
            paused = false;
            gfx::returnFromScene();
        }
    }
}
