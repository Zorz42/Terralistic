//
//  pauseScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "core.hpp"

#include "pauseScreen.hpp"
#include "game.hpp"
#include "init.hpp"

// pause screen is very simple for now

static gfx::button resume_button, quit_button;
static bool paused = false;

#define PADDING 20

INIT_SCRIPT
    resume_button.scale = 3;
    resume_button.setTexture(gfx::renderText("Resume", {255, 255, 255}));
    resume_button.x = PADDING;
    resume_button.y = PADDING;

    quit_button.scale = 3;
    quit_button.setTexture(gfx::renderText("Save & Quit", {255, 255, 255}));
    quit_button.x = PADDING;
    quit_button.y = short(resume_button.getHeight() + 2 * PADDING);
INIT_SCRIPT_END

void pauseScreen::module::render() {
    if(paused) {
        gfx::render(resume_button);
        gfx::render(quit_button);
    }
}

void pauseScreen::module::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_ESCAPE)
        paused = !paused;
    else if(key == gfx::KEY_MOUSE_LEFT) {
        if(resume_button.isHovered())
            paused = false;
        else if(quit_button.isHovered()) {
            paused = false;
            gfx::returnFromScene();
        }
    }
}
