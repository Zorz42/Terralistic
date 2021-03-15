//
//  pauseScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "core.hpp"

#include "pauseScreen.hpp"
#include "UIKit.hpp"
#include "gameLoop.hpp"
#include "init.hpp"

// pause screen is very simple for now

ui::button resume_button(ogl::top_left), quit_button(ogl::top_left);

#define PADDING 20

INIT_SCRIPT
    /*resume_button.setColor(0, 0, 0);
    resume_button.setHoverColor(100, 100, 100);
    resume_button.setScale(3);
    resume_button.setText("Resume", 255, 255, 255);
    resume_button.x = PADDING;
    resume_button.y = PADDING;

    quit_button.setColor(0, 0, 0);
    quit_button.setHoverColor(100, 100, 100);
    quit_button.setScale(3);
    quit_button.setText("Save & Quit", 255, 255, 255);
    quit_button.x = PADDING;
    quit_button.y = short(resume_button.getHeight() + 2 * PADDING);*/
INIT_SCRIPT_END

void pauseScreen::render() {
    resume_button.render();
    quit_button.render();
}

void pauseScreen::handleEvents(SDL_Event& event) {
    if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE) {
        paused = !paused;
    }
    else if(paused) {
        if(resume_button.isPressed(event))
            paused = false;
        else if(quit_button.isPressed(event)) {
            paused = false;
            gameLoop::running = false;
        }
    }
}
