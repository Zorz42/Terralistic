#include "pauseScreen.hpp"
#include <cmath>

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.renderText("Resume");
    resume_button.y = SPACING;

    quit_button.scale = 3;
    quit_button.renderText("Leave Game");
    quit_button.y = short(resume_button.getHeight() + 2 * SPACING);
    
    int width = quit_button.getWidth() + 2 * SPACING;
    back_rect.setWidth(width);
    back_rect.c.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    back_rect.setX(-width - 200);
    back_rect.smooth_factor = 3;
    back_rect.blur_intensity = BLUR;
}

void PauseScreen::render() {
    back_rect.setHeight(gfx::getWindowHeight());
    fade_rect.setWidth(gfx::getWindowWidth());
    fade_rect.setHeight(gfx::getWindowHeight());
    fade_rect.c.a = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * 70;
    fade_rect.blur_intensity = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * BLUR / 2;
    if(fade_rect.blur_intensity < 0.5f)
        fade_rect.blur_intensity = 0;
    fade_rect.render();
    back_rect.render();
    resume_button.x = back_rect.getX() + SPACING;
    quit_button.x = back_rect.getX() + SPACING;
    resume_button.render();
    quit_button.render();
}

void PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE)
        togglePause();
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered())
            togglePause();
        else if(quit_button.isHovered()) {
            paused = false;
            gfx::returnFromScene();
        }
    }
}

void PauseScreen::togglePause() {
    paused = !paused;
    disable_events = paused;
    back_rect.setX(paused ? 0 : -back_rect.getWidth() - 200);
}
