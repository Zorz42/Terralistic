//
//  pauseScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "pauseScreen.hpp"
#include "UIKit.hpp"
#include "gameLoop.hpp"

ui::button resume_button(ogl::objectType::top_left), quit_button(ogl::objectType::top_left);

#define PADDING 20

void pauseScreen::init() {
    resume_button.setColor(0, 0, 0);
    resume_button.setHoverColor(100, 100, 100);
    resume_button.setTextColor(255, 255, 255);
    resume_button.setScale(3);
    resume_button.setText("Resume");
    resume_button.setX(PADDING);
    resume_button.setY(PADDING);

    quit_button.setColor(0, 0, 0);
    quit_button.setHoverColor(100, 100, 100);
    quit_button.setTextColor(255, 255, 255);
    quit_button.setScale(3);
    quit_button.setText("Save & Quit");
    quit_button.setX(PADDING);
    quit_button.setY(resume_button.getHeight() + 2 * PADDING);
}

void pauseScreen::render() {
    resume_button.render();
    quit_button.render();
}

bool pauseScreen::handleEvents(SDL_Event& event) {
    if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE) {
        paused = !paused;
        return true;
    }
    else if(paused) {
        if(resume_button.isPressed(event))
            paused = false;
        else if(quit_button.isPressed(event)) {
            paused = false;
            gameLoop::running = false;
        }
    }
    return false;
}
