//
//  pauseScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "pauseScreen.hpp"
#include <cmath>

// pause screen is very simple for now

#define PADDING 20

void pauseScreen::init() {
    resume_button.scale = 3;
    resume_button.renderText("Resume", {255, 255, 255});
    resume_button.y = PADDING;

    quit_button.scale = 3;
    quit_button.renderText("Leave Game", {255, 255, 255});
    quit_button.y = short(resume_button.getHeight() + 2 * PADDING);
    
    back_rect.w = quit_button.getWidth() + 2 * PADDING;
    back_rect.c = {0, 0, 0};
    
    x_to_be = -back_rect.w;
    back_rect.x = x_to_be;
}

void pauseScreen::render() {
    if(back_rect.x != -back_rect.w || x_to_be != -back_rect.w) {
        back_rect.x += floor(float(x_to_be - back_rect.x) / 2.0f);
        resume_button.x = back_rect.x + PADDING;
        quit_button.x = back_rect.x + PADDING;
        back_rect.h = gfx::getWindowHeight();
        gfx::Rect(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight(), {0, 0, 0, (unsigned char)(float(back_rect.w + back_rect.x) / (float)back_rect.w * 150)}).render();
        back_rect.render();
        resume_button.render();
        quit_button.render();
    }
}

void pauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        paused = !paused;
        x_to_be = !paused * (-back_rect.w);
        disable_events = paused;
    }
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered())
            paused = false;
        else if(quit_button.isHovered()) {
            paused = false;
            gfx::returnFromScene();
        }
    }
}
