#include "pauseScreen.hpp"
#include <cmath>

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.renderText("Resume");
    resume_button.y = SPACING;

    quit_button.scale = 3;
    quit_button.renderText("Leave Game");
    quit_button.y = short(resume_button.getHeight() + 2 * SPACING);
    
    int x_to_be = -(quit_button.getWidth() + 2 * SPACING);
    back_rect.setWidth(-x_to_be);
    back_rect.c.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    back_rect.setX(x_to_be);
    back_rect.smooth_factor = 2;
}

void PauseScreen::render() {
    resume_button.x = back_rect.getX() + SPACING;
    quit_button.x = back_rect.getX() + SPACING;
    back_rect.setHeight(gfx::getWindowHeight());
    fade_rect.setWidth(gfx::getWindowWidth());
    fade_rect.setHeight(gfx::getWindowHeight());
    fade_rect.c.a = float(back_rect.getWidth() + back_rect.getX()) / (float)back_rect.getWidth() * 70;
    fade_rect.blur_intensity = float(back_rect.getWidth() + back_rect.getX()) / (float)back_rect.getWidth() * BLUR;
    if(fade_rect.blur_intensity < 0.5f)
        fade_rect.blur_intensity = 0;
    fade_rect.render();
    back_rect.render();
    resume_button.render();
    quit_button.render();
}

void PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        paused = !paused;
        back_rect.setX(!paused * (-back_rect.getWidth()));
        disable_events = paused;
    }
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered()) {
            paused = false;
            back_rect.setX(!paused * (-back_rect.getWidth()));
            disable_events = paused;
        }
        else if(quit_button.isHovered()) {
            paused = false;
            gfx::returnFromScene();
        }
    }
}
