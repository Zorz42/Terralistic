//
//  startMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "startMenu.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"

// this is the main menu, which you see on the start of the app

void startMenu::init() {
    singleplayer_button.scale = 3;
    singleplayer_button.setTexture(gfx::renderText("Singleplayer", {255, 255, 255}));
    singleplayer_button.y = short(-singleplayer_button.getTranslatedRect().h - 5);
    singleplayer_button.orientation = gfx::center;

    multiplayer_button.scale = 3;
    multiplayer_button.setTexture(gfx::renderText("Multiplayer", {255, 255, 255}));
    multiplayer_button.orientation = gfx::center;

    exit_button.scale = 3;
    exit_button.setTexture(gfx::renderText("Exit", {255, 255, 255}));
    exit_button.y = short(exit_button.getTranslatedRect().h + 5);
    exit_button.orientation = gfx::center;
}

void startMenu::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(exit_button.isHovered())
            gfx::returnFromScene();
        else if(singleplayer_button.isHovered())
            gfx::switchScene(new worldSelector());
        else if(multiplayer_button.isHovered())
            gfx::switchScene(new multiplayerSelector());
    }
}

void startMenu::render() {
    gfx::render(singleplayer_button);
    gfx::render(multiplayer_button);
    gfx::render(exit_button);
}
