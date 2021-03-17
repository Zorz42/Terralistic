//
//  startMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "core.hpp"

#include "startMenu.hpp"
#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"
#include "gameLoop.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"
#include "main.hpp"

#undef main

// this is the main menu, which you see on the start of the app

gfx::button singleplayer_button, multiplayer_button, exit_button;
gfx::textInput test;

INIT_SCRIPT
    singleplayer_button.scale = 3;
    singleplayer_button.setSurface(gfx::renderText("Singleplayer", {255, 255, 255}));
    singleplayer_button.y = short(-singleplayer_button.getRect().h - 5);
    singleplayer_button.orientation = gfx::center;

    multiplayer_button.scale = 3;
    multiplayer_button.setSurface(gfx::renderText("Multiplayer", {255, 255, 255}));
    multiplayer_button.orientation = gfx::center;

    exit_button.scale = 3;
    exit_button.setSurface(gfx::renderText("Exit", {255, 255, 255}));
    exit_button.y = short(exit_button.getRect().h + 5);
    exit_button.orientation = gfx::center;

    test.setPos(10, 10);
    test.scale = 3;
    test.setText("");
    test.textProcessing = [](char c){
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '_')
            return c;
        if(c == ' ')
            return '-';
        return '\0';
    };
INIT_SCRIPT_END

void startMenu::scene::init() {
    text_inputs = {&test};
}

void startMenu::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(exit_button.isHovered())
            gfx::returnFromScene();
        else if(singleplayer_button.isHovered())
            gfx::switchScene(new worldSelector::scene());
    }
}

void startMenu::scene::render() {
    gfx::render(singleplayer_button);
    gfx::render(multiplayer_button);
    gfx::render(exit_button);
    gfx::render(test);
}
